use std::env;
use std::fs;

fn load_input(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Could not read the file");
    return contents;
}

fn parse_input(input: &String) -> Vec<[i64;2]>{
    let mut result = Vec::new();
    let split: Vec<&str> = input.lines().collect();
    for val in &split {
        let parts: Vec<&str> = val.split_whitespace().collect();
        let direction: &str = parts[0];
        let magnitude: i64 = parts[1].parse().expect("Could not parse to an int");
        match direction {
            "forward" => result.push([magnitude, 0_i64]),
            "up" => result.push([0_i64, -magnitude]),
            "down" => result.push([0_i64, magnitude]),
            _ => println!("Could not parse {}", direction)
        }
    }
    return result;
}

fn sum_vectors<const N: usize>(data: &Vec<[i64; N]>) -> [i64; N]{
    let mut result = [0_i64; N];
    for vals in data {
        for idx in 0..vals.len() {
            result[idx] += vals[idx];
        }
    }
    return result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = load_input(path);
    let parsed_input = parse_input(&input);
    let summed = sum_vectors(&parsed_input);
    println!("{:?}, which multiplied together is {}", summed, summed[0] * summed[1]);
}