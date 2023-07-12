use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input/vents";
    let lines = read_file(filename);

    let mut vents = parse_vents(lines);

    filter_lines(&mut vents);
    println!("vents {:?}", &vents);

    let mut d = create_diagram();
    for v in vents {
        d.draw_line(&v);
    }

    let overlapping_points = get_overlapping_points(&d);
    println!("overlapping points: {}", overlapping_points);
    println!("diagram:\n{}", &d);
}

#[derive(Default, Debug)]
struct Vent (pub Vec<Vec<i32>>);

impl Vent {
    fn new() -> Vent { Vent(Vec::new()) }
}

#[derive(Default, Debug)]
struct Diagram (pub Vec<Vec<String>>);

impl Diagram {
    fn new() -> Diagram {
        let mut d: Vec<Vec<String>> = Vec::new();
        for _r in 0..10 {
            let mut row: Vec<String> = Vec::new();
            for _item in 0..10 {
                row.push(".".to_string());
            }
            d.push(row);
        }

        Diagram(d)
    }

    fn get_new_diagram_value(old_val: &String) -> String {
        let new_val = "1".to_string();
        if old_val != "." {
            let old_val_num: i32 = old_val.parse().unwrap();
            let new_val_num = old_val_num + 1;
            return new_val_num.to_string()
        }

        new_val
    }

    fn draw_line(&mut self, v: &Vent) {
        let x1:i32 = v.0[0][0] as i32;
        let y1:i32 = v.0[0][1] as i32;
        let x2:i32 = v.0[1][0] as i32;
        let y2:i32 = v.0[1][1] as i32;

        let diff_x = (x1 - x2).abs() as usize;
        let diff_y = (y1 - y2).abs() as usize;

        let min_x:usize = std::cmp::min(x1 as usize, x2 as usize);
        let min_y:usize = std::cmp::min(y1 as usize, y2 as usize);

        if diff_x != 0 {
            for x in min_x..=(min_x + diff_x) {
                let old_val = &self.0[min_y][x];
                let new_val = Diagram::get_new_diagram_value(old_val);
                let _def = std::mem::replace(&mut self.0[min_y][x], new_val);
            }
        }

        if diff_y != 0 {
            for y in min_y..=(min_y+diff_y) {
                let old_val = &self.0[y][min_x];
                let new_val = Diagram::get_new_diagram_value(old_val);
                let _def = std::mem::replace(&mut self.0[y][min_x], new_val);
            }
        }
    }
}

impl std::fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut comma_separated = String::new();

        for l in &self.0[0..self.0.len()] {
            for val in l {
                comma_separated.push_str(val);
                comma_separated.push_str(" ");
            }
            comma_separated.push_str("\n");
        }

        write!(f, "{}", comma_separated)
    }
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

        let coordinates_begin:Vec<i32> = coordinates_str_begin.into_iter().map(|c| (c.parse().unwrap())).collect();
        let coordinates_end:Vec<i32> = coordinates_str_end.into_iter().map(|c| (c.parse().unwrap())).collect();

        let vent = build_vent(coordinates_begin, coordinates_end);
        vents.push(vent);
    }

    vents
}

fn build_vent(coord_begin: Vec<i32>, coord_end: Vec<i32>) -> Vent {
    let mut vent = Vent::new();
    vent.0.push(coord_begin);
    vent.0.push(coord_end);

    vent
}

fn filter_lines(vents: &mut Vec<Vent>) -> () {
    vents.retain(|v| (v.0[0].get(0).unwrap() == v.0[1].get(0).unwrap())
        || v.0[0].get(1).unwrap() == v.0[1].get(1).unwrap());
}

fn create_diagram() -> Diagram {
    Diagram::new()
}

fn get_overlapping_points(d: &Diagram) -> u32 {
    let mut count = 0;
    for row in &d.0 {
        for item in row {
            if item != "." {
                let item_number:u32 = item.parse().unwrap();
                if item_number > 1 { count+=1; }
            }
        }
    }

    count
}