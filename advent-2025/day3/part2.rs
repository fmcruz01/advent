use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    let content_str = read_to_string(filename)?;
    let total: u64 = content_str.lines().map(get_joltage).sum();
    println!("Total: {}", total);
    Ok(())
}

fn get_joltage(battery: &str) -> u64 {
    let digits: Vec<u64> = battery
        .chars()
        .map(|ch| ch.to_digit(10).expect("Expected digit") as u64)
        .collect();

    let needed = 12usize;
    assert!(digits.len() >= needed, "Expected at least 12 digits");

    // Greedy: for each position, pick the max digit that leaves enough
    // remaining digits to complete the 12-digit result. On ties, prefer the
    // rightmost max to keep more options for later positions.
    let mut result = 0u64;
    let mut start = 0usize;
    for remaining_after in (0..needed).rev() {
        let end = digits.len() - remaining_after;
        let (offset, digit) = max_digit(&digits[start..end]);
        result = result * 10 + digit;
        start += offset + 1;
    }
    result
}

fn max_digit(slice: &[u64]) -> (usize, u64) {
    slice
        .iter()
        .enumerate()
        .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
        .map(|(idx, &val)| (idx, val))
        .expect("Expected more digits.")
}
