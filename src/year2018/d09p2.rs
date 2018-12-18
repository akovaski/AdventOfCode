use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use super::d09p1;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day09.txt")?;
    let mut reader = BufReader::new(f);

    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let cap = re.captures(&buf).unwrap();

    let num_players: usize = cap[1].parse().unwrap();
    let last_marble: usize = cap[2].parse::<usize>().unwrap() * 100;

    println!(
        "{}",
        d09p1::simulate_highest_score(num_players, last_marble)
    );
    Ok(())
}
