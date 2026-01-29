use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let ranges = read_input(filename).unwrap_or(vec![]);
    let mut total: u64 = 0;
    for range in ranges {
        let boundaries: Vec<String> = range.split('-').map(str::to_owned).collect();
        println!("Range: {} - {}", boundaries[0], boundaries[1]);
        let mut id: u64 = boundaries[0].parse().expect("Value should be an integer");
        let last_id: u64 = boundaries[1].parse().expect("Value should be an integer");
        while id <= last_id {
            if is_invalid(id) {
                println!("ID {} is invalid, adding to total {}", id, total);
                total += id;
            }
            id += 1;
        }
    }
    println!("Total {}", total);
}

fn read_input<P>(filename: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let contents = read_to_string(filename)?;
    let ranges: Vec<String> = contents.trim_end().split(',').map(str::to_owned).collect();
    Ok(ranges)
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
            println!("ID: {} has equal parts for divisor: {}", id, divisor);
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
