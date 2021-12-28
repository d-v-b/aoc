use std::env;
use std::fs;

fn load_input(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Could not read the file");
    return contents;
}

fn parse_input(input: &String) -> Vec<&str>{
    let result: Vec<&str> = input.lines().collect();
    return result;
}

fn compute_power_consumption(data: Vec<&str>) -> u64{
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    let mut buf = vec![0_i64];
    for element in data {
        for (idx, char) in element.chars().enumerate() {
            if buf.len() == idx {
                buf.push(0)
            }
            match char{
                '0' => buf[idx] -= 1,
                '1' => buf[idx] += 1,
                _ => println!("Invalid input: {}", char)
            }
        }
    }
    println!("{:?}", buf);
    for (idx, element) in buf.iter().enumerate() {
        let val = u64::pow(2, (buf.len() - idx - 1) as u32);
        println!("{}", val);
        if *element < 0 {
            gamma += val;
        }
        else {
            epsilon += val;
        }
    }
    println!("gamma: {}, epsilon: {}", gamma, epsilon);
    gamma * epsilon
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = load_input(path);
    let parsed_input = parse_input(&input);
    let power = compute_power_consumption(parsed_input);
    println!("power: {:?}", power);
}