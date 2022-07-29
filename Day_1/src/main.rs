use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/measurements";
    let lines = read_file(filename);

    let measurements: Vec<i64> = lines.iter().map(|l| l.parse::<i64>().unwrap()).collect();

    let mut def = measurements[0];
    let mut count = 0;
    for m in measurements {
        if m > def {
            count += 1;
        }
        def = m;
    }

    println!("Result: {}", count);
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}