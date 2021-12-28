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
    let result = count_increments(&parsed_input);
    println!("{}", result);
}   
