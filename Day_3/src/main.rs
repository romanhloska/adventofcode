use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/report";
    let lines = read_file(filename);


    let report = parse_report(lines);

    let width = report.0[0].0.len();
    println!("width = {}", width);
    for col in 0..width {
        println!("{:?}. step", col);
        for row in &report.0 {
            println!("{}", row.0[col]);
        }
        println!("--------------------");
    }


    // for r in &report.0 {
    //     for l in &r.0[0..r.0.len()] {
    //         //print!("{}", l);
    //         &report.0[l];
    //
    //     }
    //     //println!("\n-------------------");
    //     //println!("{}", r.0[0]);
    // }

    //println!("{}", report);

}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}

struct NumVec (pub Vec<i32>);

impl NumVec {
    fn new() -> NumVec {
        NumVec(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }

    fn get(&mut self, i: usize) {
        self.0.get(i);
    }
}

impl FromIterator<i32> for NumVec {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let mut c = NumVec::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

struct Report (pub Vec<NumVec>);

impl std::fmt::Display for NumVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", comma_separated)
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, l| {
            result.and_then(|_| writeln!(f, "{}", l))
        })
    }
}

fn parse_report(lines: Vec<String>) -> Report {
    let mut reps: Vec<NumVec>  = Vec::new();

    for l in lines {
        let v:NumVec = l.chars().map(|c| (c.to_digit(10).unwrap() as i32)).collect::<NumVec>();
        reps.push(v);
    }

    let cs = Report(reps);

    cs
}