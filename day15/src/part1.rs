use std::fs;

fn hash(input: &str) -> u32 {
    let mut value: u32 = 0;
    for ch in input.chars() {
        value += ch as u32;
        value *= 17;
        value %= 256;
    }
    return value;
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("File not found");
    let original_input: Vec<&str> = contents.lines().collect();
    let split_input: Vec<&str> = original_input[0].split(",").collect();

    let mut result: u32 = 0;
    for i in 0..split_input.len() {
        result += hash(split_input[i]);
    }

    println!("{:?}", result);
}
