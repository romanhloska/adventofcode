use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/measurements";
    let lines = read_file(filename);


    for line in lines {
    }

    println!("Result: ");
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}