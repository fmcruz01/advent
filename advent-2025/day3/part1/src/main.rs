use std::env::args;
use std::fs::File;
use std::fs::read_to_string;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            total += get_joltage(line);
        }
    }
    println!("Total: {}", total);
}

fn read_input<P>(filename: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let contents = read_to_string(filename)?;
    let ranges: Vec<String> = contents.trim_end().split(',').map(str::to_owned).collect();
    Ok(ranges)
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_joltage(battery: String) -> u32 {
    let mut str_iter = battery.char_indices();
    let mut first_digit = (0, 0);
    let mut second_digit = (0, 0);
    let len = battery.len();
    while let Some((i, ch)) = str_iter.next() {
        let curr = ch.to_digit(10u32).unwrap();
        first_digit = if first_digit.1 < curr && i < len - 1 {
            (i, curr)
        } else {
            first_digit
        }
    }
    let mut str_iter_second = battery.char_indices().skip(first_digit.0 + 1);
    while let Some((i, ch)) = str_iter_second.next() {
        let curr = ch.to_digit(10u32).unwrap();
        second_digit = if second_digit.1 < curr {
            (i, curr)
        } else {
            second_digit
        }
    }
    let result: u32 = (first_digit.1 * 10) + second_digit.1;
    println!("Battery: {}", result);
    result
}
