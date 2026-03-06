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
            let (start_str, end_str) = range.split_once('-').expect("Expected '-' between ranges.");
            let start: u64 = start_str.parse().expect("Value should be an integer");
            let end: u64 = end_str.parse().expect("Value should be an integer");
            (start..=end).filter(|&id| is_invalid(id)).sum::<u64>()
        }).sum();
    println!("Total {}", total);
    Ok(())
}

fn is_invalid(id: u64) -> bool {
    let power: u32 = id.checked_ilog10().unwrap_or(0);
    let mut divisors: Vec<u32> = divisors::get_divisors(power.checked_add(1).unwrap_or(0));
    if power == 1 {
        divisors = Vec::new();
    }
    if power != 0 {
        divisors.push(1);
    }
    for divisor in divisors {
        if check_eq_parts(id, power, divisor) {
            return true;
        }
    }
    false
}

fn check_eq_parts(id: u64, power: u32, divisor: u32) -> bool {
    let mut i: u32 = 2;
    let power_incr = power.checked_add(1).unwrap_or(0);
    let first = id / 10u64.pow(power_incr-divisor);
    let mut next: u64;
    while  i*divisor <= power_incr {
        next = (id / 10u64.pow(power_incr-(i*divisor))) % 10u64.pow(divisor);
        if next != first {
            return false;
        }
        // End loop by incrementing
        i += 1;
    }
    true
}
