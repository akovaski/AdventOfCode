use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day01.txt")?;
    let mut reader = BufReader::new(f);
    let mut freq = 0;

    let drift_iter = reader
        .by_ref()
        .lines()
        .map(|l| l.unwrap().trim().parse::<i32>().unwrap());

    for drift in drift_iter {
        freq += drift;
    }
    println!("{}", freq);

    Ok(())
}
