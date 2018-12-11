use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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
                id: cap[1].parse().unwrap(),
                x: cap[2].parse().unwrap(),
                y: cap[3].parse().unwrap(),
                width: cap[4].parse().unwrap(),
                height: cap[5].parse().unwrap(),
            }
        })
        .collect();

    let mut fabric = HashMap::new();
    for claim in &claim_vec {
        for x in (claim.x)..(claim.x + claim.width) {
            for y in (claim.y)..(claim.y + claim.height) {
                let layers = fabric.entry((x, y)).or_insert(0);
                *layers += 1;
            }
        }
    }
    for claim in &claim_vec {
        let mut no_count = false;
        for x in (claim.x)..(claim.x + claim.width) {
            for y in (claim.y)..(claim.y + claim.height) {
                let layers = fabric.get(&(x, y)).unwrap();
                if *layers > 1 {
                    no_count = true;
                    break;
                }
            }
            if no_count {
                break;
            }
        }
        if no_count == false {
            println!("{}", claim.id);
        }
    }

    Ok(())
}
