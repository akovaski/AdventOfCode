use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    assert!(lines.len() == 2);

    let earliest: i32 = lines[0].parse().unwrap();
    let busses: Vec<i32> = lines[1].split(',').filter(|b| *b != "x").map(|b| b.parse::<i32>().unwrap()).collect();

    let wait_times: Vec<i32> = busses.iter().map(|b| b - (earliest % b)).collect();

    let (min_bus, min_wait) = busses.iter().zip(wait_times.iter()).min_by_key(|&(_, wt)| wt).unwrap();

    println!("magic number: {}", min_bus * min_wait);


    Ok(())
}

