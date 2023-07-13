use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/lanternfish";
    let lines = read_file(filename);
    println!("{:?}", lines);

    let lf = parse_lanternfish(lines);
    println!("{:?}", lf);

}

#[derive(Default, Debug)]
struct Lanternfish {
    age: u32,
    def_days_to_new: u32,
}

impl Lanternfish {
    fn new(age: u32) -> Lanternfish {
        Lanternfish {
            age,
            def_days_to_new: 6,
        }
    }
}
fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn parse_lanternfish(lines: Vec<String>) -> Vec<Lanternfish> {
    let mut lanternfish: Vec<Lanternfish> = Vec::new();

    for l in lines {
        let numbers:Vec<&str> = l.split(",").collect();
        for number in numbers {
            let mut lf = Lanternfish::new(number.parse().expect("Not a number!"));
            lanternfish.push(lf);
        }
    }

    lanternfish
}
