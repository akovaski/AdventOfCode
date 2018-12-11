use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day01.txt")?;
    let mut reader = BufReader::new(f);
    let mut freq = 0;
    let mut freq_set = HashSet::new();

    let drift_lines: Vec<i32> = reader
        .by_ref()
        .lines()
        .map(|l| l.unwrap().trim().parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    freq_set.insert(freq);
    for drift in drift_lines.iter().cycle() {
        count += 1;
        freq += drift;
        if freq_set.contains(&freq) {
            break;
        }
        freq_set.insert(freq);
    }
    println!("count: {}", count);
    println!("{}", freq);

    Ok(())
}
