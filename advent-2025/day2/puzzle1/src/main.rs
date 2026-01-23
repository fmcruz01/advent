use std::env::args;
use std::fs::{read_to_string};
use std::io::{ Result };
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let ranges = read_input(filename).unwrap_or(vec![]);
    let mut total: u32 = 0;
    for range in ranges {
        let boundaries: Vec<String> = range.split('-').map(str::to_owned).collect();
        println!("Ranbge: {} - {}", boundaries[0], boundaries[1]);
        let mut id = boundaries[0].parse().expect("Value should be an integer");
        let last_id = boundaries[1].parse().expect("Value should be an integer");
        while id <= last_id {
            if is_invalid(id){
                total += id;
            }
            id += 1;
        }
    }
    println!("Total {}", total);
}


fn read_input<P>(filename: P) -> Result<Vec<String>> where P: AsRef<Path>, {
    let contents = read_to_string(filename)?;
    let ranges: Vec<String> = contents.trim_end().split(',').map(str::to_owned).collect();
    Ok(ranges)
}

fn is_invalid(id: u32) -> bool {
    let powers = id.checked_ilog10().unwrap_or(0);
    if powers % 2 == 0 {
        return false
    }
    let factor = (powers / 2).checked_add(1).unwrap_or(0);
    let left = id / 10u32.pow(factor);
    let right = id % 10u32.pow(factor);
    if left == right {
        return true
    }
    false
}
