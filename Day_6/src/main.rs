use std::fs::File;
use std::io::{BufRead, BufReader};

const DAYS: i32 = 80;

fn main() {
    let filename = "src/input/lanternfish";
    let lines = read_file(filename);

    let mut lf = parse_lanternfish(lines);
    println!("Initial state: {:?}", lf);

    for _current_day in 0..DAYS {
        process_day(&mut lf);
        // println!("After {:?} day: {:?}", (_current_day + 1), lf);
    }

    println!("After {:?} there are {:?} lanternfish", DAYS , lf.len());
}

#[derive(Default, Debug)]
struct Lanternfish {
    counter: i32,
}

impl Lanternfish {
    fn new(counter: i32) -> Lanternfish {
        Lanternfish {
            counter,
        }
    }

    fn get_next_day(&mut self) -> i32 {
        if self.counter > 0 {
            self.counter-=1;
            return self.counter;
        }

        self.reset();

        -1
    }

    fn reset(&mut self) {
        self.counter = 6;
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
            let lf = Lanternfish::new(number.parse().expect("Not a number!"));
            lanternfish.push(lf);
        }
    }

    lanternfish
}

fn process_day(lanternfish: &mut Vec<Lanternfish>) {
    for index_lf in 0..lanternfish.len() {
        let internal_lf_counter = lanternfish[index_lf].get_next_day();

        if internal_lf_counter == -1 {
            let lf_born = Lanternfish::new(8);
            lanternfish.push(lf_born);
        }
    }
}
