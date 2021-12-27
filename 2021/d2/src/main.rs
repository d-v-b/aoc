use std::env;
use std::fs;

fn load_input(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Could not read the file");
    return contents;
}

fn parse_input(input: &String) -> Vec<u16>{
    let mut result = Vec::new();
    let split: Vec<&str> = input.split("\n").collect();
    for val in &split {
        let pos = val.parse::<u16>();
        match pos {
            Ok(val) => result.push(val),
            Err(e) => println!("{}", e), 
        }
    }
    return result;
}

fn sum(data: &[u16]) -> u16 {
    let mut result: u16 = 0;
    for element in data {
        result += *element;
    }
    return result
}

fn windowed_sum(data: &Vec<u16>, window_size: u8) -> Vec<u16> {
    let trim: usize = data.len() % window_size as usize;
    println!("the data has length {}, and trim is {}", data.len(), trim);
    let numel = data.len() - trim;
    let mut result = vec![0_u16; numel];
    for (idx, window) in data.windows(window_size.into()).enumerate(){
        result[idx] = sum(window) as u16;
        }

    return result
}

fn count_increments(measurements: &Vec<u16>) -> u32{
    let mut result: u32 = 0;
    for (idx, val) in measurements.iter().enumerate() {
        if idx > 0 {
            if val > &measurements[idx - 1] {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = load_input(path);
    let parsed_input = parse_input(&input);
    println!("The first 10 values are {:?}", &parsed_input[0..10]);
    let smoothed_input = windowed_sum(&parsed_input, 3);
    println!("The first 10 smoothed values are {:?}", &smoothed_input[0..10]);
    let result = count_increments(&smoothed_input);
    println!("The result is {}", result);
}
