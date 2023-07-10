use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/vents";
    let lines = read_file(filename);

    let vents = parse_vents(lines);

    println!("vents {:?}", vents);
}

#[derive(Default, Debug)]
struct Vent (pub Vec<Vec<u32>>);

impl Vent {
    fn new() -> Vent { Vent(Vec::new()) }
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn parse_vents(lines: Vec<String>) -> Vec<Vent>{
    let mut vents: Vec<Vent>  = Vec::new();

    for l in lines {
        let segment:Vec<&str> = l.split("->").collect();
        let coordinates_str_begin:Vec<&str> = segment[0].trim().split(",").collect();
        let coordinates_str_end:Vec<&str> = segment[1].trim().split(",").collect();

        let coordinates_begin:Vec<u32> = coordinates_str_begin.into_iter().map(|c| (c.parse().unwrap())).collect();
        let coordinates_end:Vec<u32> = coordinates_str_end.into_iter().map(|c| (c.parse().unwrap())).collect();

        let vent = build_vent(coordinates_begin, coordinates_end);
        vents.push(vent);
    }

    vents
}

fn build_vent(coord_begin: Vec<u32>, coord_end: Vec<u32>) -> Vent {
    let mut vent = Vent::new();
    vent.0.push(coord_begin);
    vent.0.push(coord_end);

    vent
}