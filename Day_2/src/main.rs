use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/commands";
    let lines = read_file(filename);

    let cmds = parse_commands(lines);

    println!("{}", cmds);

    let (mut h, mut d) = (0, 0);
    for cmd in cmds.0 {
        if cmd.direction == "forward" {
            h+=cmd.unit;
        } else if cmd.direction == "up" {
            d-=cmd.unit;
        } else {
            d+=cmd.unit;
        }
    }

    println!("horizontal = {}", h);
    println!("depth = {}", d);

    println!("Result: {}", h*d);
}

struct Command {
    direction: String,
    unit: i64,
}

struct Commands (pub Vec<Command>);

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.direction, self.unit)
    }
}

impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, command| {
            result.and_then(|_| writeln!(f, "{}", command))
        })
    }
}

fn parse_commands(lines: Vec<String>) -> Commands {
    let mut cmds: Vec<Command>  = Vec::new();

    for l in lines {
        let v:Vec<&str> = l.split(" ").collect();
        let c = build_command(String::from(v[0]), (v[1]).parse::<i64>().unwrap());
        cmds.push(c);
    }

    let cs = Commands(cmds);

    cs
}

fn build_command(direction: String, unit: i64) -> Command {
    Command {
        direction,
        unit,
    }
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}