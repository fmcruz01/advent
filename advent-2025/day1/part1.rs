use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let file_path = &args[1];
    let lines = read_lines(file_path)?;
    let zero_counter = lines
        .map_while(Result::ok)
        .fold((Dial::new(), 0u32), |(mut dial, count), line| {
            let (direction, value_str) = line.split_at(1);
            let direction = direction.chars().next().unwrap();
            let value: u32 = value_str.parse().expect("Value should be an integer.");

            let real_val: u32 = if direction == 'L' {
                100 - (value % 100)
            } else {
                value % 100
            };

            dial.spin(real_val as usize);
            println!("Dial spinned to: {}", dial.pos);

            let hit_zero = dial.pos == 0;
            (dial, count + if hit_zero { 1 } else { 0 })
        })
        .1;
    println!("Final value: {}", zero_counter);
    Ok(())
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

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
