use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let ranges = read_to_string(filename)?;
    let total: u64 = ranges
        .trim_end()
        .split(',')
        .map(|range| {
            let (start_str, end_str) = range.split_once('-').expect("Range should contain '-'");
            let start: u64 = start_str.parse().expect("Value should be an integer");
            let end: u64 = end_str.parse().expect("Value should be an integer");
            (start..=end).filter(|&id| is_palindrome(id)).sum::<u64>()
        })
        .sum();
    println!("Total {}", total);
    Ok(())
}

fn is_palindrome(id: u64) -> bool {
    let powers = id.checked_ilog10().unwrap_or(0);
    if powers % 2 == 0 {
        return false;
    }
    let factor = (powers / 2).checked_add(1).unwrap_or(0);
    let left = id / 10u64.pow(factor);
    let right = id % 10u64.pow(factor);
    if left == right {
        // println!("Id {} is invalid", id);
        // println!("Id {} is valid", id);
        return true;
    }
    // println!("Id {} is invalid", id);
    false
}
