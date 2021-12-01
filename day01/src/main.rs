use std::fs;

const INPUT_FILE_PATH: &str = "data/input.txt";
const READ_FAILURE_MSG: &str = "Unable to read file";

fn main() {
    let data = fs::read_to_string(INPUT_FILE_PATH).expect(READ_FAILURE_MSG);
    let lines: Vec<&str> = data.split('\n').filter(|line| !line.is_empty()).collect();
    let depths: Vec<i32> = lines.iter().map(|line| line.parse::<i32>().unwrap()).collect();
    let triples = triples(&depths);

    println!("number of single-value depth increases: {}", num_increases(depths));
    println!("number of triple-value depth increases: {}", num_increases(triples));
}

fn num_increases(depths: Vec<i32>) -> i32 {
    let mut increases = 0;

    for index in 0..depths.len()-1 {
        let depth = depths[index];
        let next_depth = depths[index + 1];
        
        if next_depth > depth {
            increases += 1;
        }
    }

    increases
}

fn triples(depths: &[i32]) -> Vec<i32> {
    let mut triples = Vec::new();

    for index in 0..depths.len()-2 {
        let first = depths[index];
        let second = depths[index + 1];
        let third = depths[index + 2];
        triples.push(first + second + third);
    }

    triples
}
