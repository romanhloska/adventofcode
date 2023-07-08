use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/report";
    let lines = read_file(filename);


    let report = parse_report(lines);

    let (gamma, epsilon) = calculate_gamma_epsilon_rate(&report);

    let gamma_bin = transform_to_binary(gamma);
    let epsilon_bin = transform_to_binary(epsilon);

    let gamma_dec = i32::from_str_radix(&gamma_bin, 2).expect("Not a binary number!");
    let epsilon_dec = i32::from_str_radix(&epsilon_bin, 2).expect("Not a binary number!");

    println!("gamma (binary, hex): {:?} -> {:?}", gamma_bin, gamma_dec);
    println!("epsilon (binary, hex): {:?} -> {:?}", epsilon_bin, epsilon_dec);

    println!("submarine consumption: {}", (gamma_dec * epsilon_dec));
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

fn calculate_gamma_epsilon_rate(report: &Report) -> (Vec<i32>, Vec<i32>) {
    let bin_num_size = report.0[0].0.len();
    let mut gamma = vec![-1; bin_num_size];
    let mut epsilon = vec![-1; bin_num_size];
    let (mut count_bit0, mut count_bit1) = (0, 0);

    // calculate the gamma and epsilon rate
    for col in 0..bin_num_size {
        for row in &report.0 {
            if row.0[col] == 0 {
                count_bit0+=1;
            } else {
                count_bit1+=1;
            }
        }

        if count_bit0 > count_bit1 {
            gamma[col] = 0;
            epsilon[col] = 1;
        } else {
            gamma[col] = 1;
            epsilon[col] = 0;
        }

        count_bit0 = 0;
        count_bit1 = 0;
    }

    (gamma, epsilon)
}

fn transform_to_binary(val: Vec<i32>) -> String {
    let val_to_string: Vec<String> = val.iter().map(|v| v.to_string()).collect();

    val_to_string.join("")
}