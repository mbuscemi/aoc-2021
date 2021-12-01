use std::fs;

const INPUT_FILE_PATH: &str = "data/input.txt";
const READ_FAILURE_MSG: &str = "Unable to read file";

fn main() {
    let data = fs::read_to_string(INPUT_FILE_PATH).expect(READ_FAILURE_MSG);
    println!("Input data: {}", data);
}
