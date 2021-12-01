use std::fs;

const INPUT_FILE_PATH: &str = "data/input.txt";
const READ_FAILURE_MSG: &str = "Unable to read file";

fn main() {
    let data = fs::read_to_string(INPUT_FILE_PATH).expect(READ_FAILURE_MSG);
    let lines: Vec<&str> = data.split('\n').filter(|line| !line.is_empty()).collect();
    let depths: Vec<i32> = lines.iter().map(|line| line.parse::<i32>().unwrap()).collect();

    println!("number of increases: {}", num_increases(depths));
}

fn num_increases(depths: Vec<i32>) -> i32 {
    let mut increases = 0;

    for (index, depth) in depths.iter().enumerate() {
        if index < depths.len() - 1 {
            let next_depth = depths[index + 1];
            if next_depth > *depth {
                increases += 1;
            }
        }
    }

    increases
}