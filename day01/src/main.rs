use std::fs;

const INPUT_FILE_PATH: &str = "data/input.txt";
const READ_FAILURE_MSG: &str = "Unable to read file";

fn main() {
    let data = fs::read_to_string(INPUT_FILE_PATH).expect(READ_FAILURE_MSG);
    let lines: Vec<&str> = data.split('\n').filter(|line| !line.is_empty()).collect();
    let depths: Vec<i32> = lines.iter().map(|line| line.parse::<i32>().unwrap()).collect();

    for depth in depths {
        println!("{}", depth);
    }
}
