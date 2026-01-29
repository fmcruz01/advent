use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];

    let mut zero_counter: u32 = 0;
    
    let mut dial = Dial::new();
   
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let mut line_iter = line.chars();
            println!("Reading: {}", line_iter.as_str());
            let direction = line_iter.next().unwrap();
            let value: u32 = line_iter.as_str().parse().expect("Value should be an integer.");
            
            let real_val: u32 = if direction.eq(&'L') {
                100 - (value % 100)
            } else {
                value % 100
            }; 
            
            dial.spin(real_val as usize);
            println!("Dial spinned to: {}", dial.pos);

            if dial.pos == 0 {
                zero_counter += 1;
            }
        }
    }

    println!("Final value: {}", zero_counter);
}

struct Dial {
    pos: usize,
}

impl Dial {
    fn new() -> Self {
        Self { pos: 50 }
    }
    
    fn spin(&mut self, steps: usize) {
        self.pos = (self.pos + steps) % 100;
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
