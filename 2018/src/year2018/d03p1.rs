use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day03.txt")?;
    let mut reader = BufReader::new(f);

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claim_vec: Vec<_> = reader
        .by_ref()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let line_ref = line.trim().as_ref();
            let cap = re.captures(line_ref).unwrap();
            Claim {
                id: cap[1].parse::<usize>().unwrap(),
                x: cap[2].parse::<usize>().unwrap(),
                y: cap[3].parse::<usize>().unwrap(),
                width: cap[4].parse::<usize>().unwrap(),
                height: cap[5].parse::<usize>().unwrap(),
            }
        })
        .collect();

    let mut fabric = HashMap::new();
    let mut sq_inch_multi_claims = 0;
    for claim in claim_vec {
        for x in (claim.x)..(claim.x + claim.width) {
            for y in (claim.y)..(claim.y + claim.height) {
                let layers = fabric.entry((x, y)).or_insert(0);
                *layers += 1;

                if *layers == 2 {
                    sq_inch_multi_claims += 1;
                }
            }
        }
    }

    println!("{}", sq_inch_multi_claims);

    Ok(())
}
