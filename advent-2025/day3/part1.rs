use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let lines = read_to_string(filename)?;
    let total: u32 = lines.lines().map(|line| get_joltage(&line)).sum();
    println!("Total: {}", total);
    Ok(())
}

fn get_joltage(battery: &str) -> u32 {
    let digits: Vec<u32> = battery
        .chars()
        .map(|ch| ch.to_digit(10).expect("Expected digit"))
        .collect();

    let (first_idx, &first_val) = digits
        .iter()
        .enumerate()
        .take(digits.len() - 1)
        .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
        .expect("Need at least 2 digits");
    let &second_val = digits[first_idx + 1..]
        .iter()
        .max()
        .expect("Need a digit after the first");

    first_val * 10 + second_val
}
