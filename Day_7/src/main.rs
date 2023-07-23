use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/crabs";
    let lines = read_file(filename);

    let crabs = parse_crabs(lines.get(0).unwrap());

    let max_crab = crabs.iter().max_by(|a,b| a.position.partial_cmp(&b.position).unwrap()).unwrap();

    let (res_position, res_consumption) = calculate_least_consumption(&crabs, max_crab);

    println!("\nThe least consumption is for {:?} position -> {:?}", res_position, res_consumption);
}

#[derive(Default, Debug)]
struct Crab {
    position: i32,
}

impl Crab {
    fn new(position: i32) -> Crab {
        Crab {
            position,
        }
    }

    fn get_consumption(&self, position: i32) -> i32 {
        (self.position - position).abs()
    }
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn parse_crabs(line: &str) -> Vec<Crab> {
    let mut crabs: Vec<Crab> = Vec::new();

    let positions: Vec<&str> = line.split(",").collect();
    for position in positions {
        let crab = Crab::new(position.parse().expect("Not a number!"));
        crabs.push(crab);
    }

    crabs
}

fn calculate_least_consumption(crabs: &Vec<Crab>, max_crab: &Crab) -> (i32, i32) {
    let mut res_position = -1;
    let mut min_consumption: i32 = (crabs.len() * max_crab.position as usize) as i32;
    for sub_pos in 0..=max_crab.position {
        let sub_sum: i32 = crabs.iter().map(|crab| crab.get_consumption(sub_pos)).sum();
        // println!("The consumption for {:?} position is {:?}", sub_pos, sub_sum);
        if sub_sum < min_consumption {
            min_consumption = sub_sum;
            res_position = sub_pos;
        }
    }

    (res_position, min_consumption)
}