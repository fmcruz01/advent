use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};
use std::path::Path;


fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];
    
    let mut final_count: u32 = 0;
    let mut dial: Dial = Dial::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            println!("Reading line: {}", line);
            let (dir, value) = parse_line(&line);
            final_count += dial.spin(value, dir);
            println!("------- Current count: {} -------", final_count);
        }
    }

    println!("Final value: {}", final_count);
}

struct Dial {
    pos: i32,
}

impl Dial {
    fn new() -> Self {
        Self { pos: 50 }
    }

    fn spin(&mut self, steps: u32, dir: i32) -> u32 {
        let mut zero_count: u32 = 0;
        let actual_step: i32 = dir * (steps as i32 % 100); 
        println!("Pos: {} - Step: {}", self.pos, actual_step);
        if self.pos + actual_step > 100  {
            println!("Passed through 0. Adding to count.");
            zero_count += 1;
            self.pos = (self.pos + actual_step) % 100;
        }else if self.pos + actual_step < 0 {
            zero_count = if self.pos != 0 {
                println!("Passed through 0. Adding to count.");
                zero_count + 1
            } else {
                zero_count
            };
            self.pos = 100 + (self.pos + actual_step) % 100;
        }else {
            self.pos = (self.pos + actual_step) % 100;
        }
        
        if self.pos == 0 {
            println!("Position is 0. Adding to count.");
            zero_count += 1;
        }
        println!("Current pos: {}", self.pos);
        let mut cur_steps = steps / 100;
        println!("Steps missing: {}", cur_steps);
        while cur_steps > 0  {
            zero_count += cur_steps as u32 % 100;
            cur_steps = cur_steps / 100;

        };
        zero_count
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn parse_line(line: &str) -> (i32, u32) {
    let mut iter = line.chars();
    let direction_char: char = iter.next().unwrap();
    let value: u32 = iter.as_str().parse().expect("Should parse to an integer.");
    let direction: i32 = if direction_char.eq(&'L') {
            -1
        } else {
            1
        };
        
    (direction, value)
}   
