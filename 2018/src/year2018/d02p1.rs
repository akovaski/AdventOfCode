use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day02.txt")?;
    let mut reader = BufReader::new(f);

    let box_vec: Vec<String> = reader.by_ref().lines().map(|l| l.unwrap()).collect();

    let mut num_two_count = 0;
    let mut num_three_count = 0;
    for id in box_vec {
        let mut counts = HashMap::new();
        for c in id.chars() {
            let c_count = counts.entry(c).or_insert(0);
            *c_count += 1;
        }

        let mut has_two_count = false;
        let mut has_three_count = false;

        for (_, count) in counts {
            if count == 2 {
                has_two_count = true;
            }
            if count == 3 {
                has_three_count = true;
            }
            if has_two_count && has_three_count {
                break;
            }
        }

        if has_two_count {
            num_two_count += 1;
        }
        if has_three_count {
            num_three_count += 1;
        }
    }

    println!("2: {}, 3:{}", num_two_count, num_three_count);
    println!(
        "{} x {} = {}",
        num_two_count,
        num_three_count,
        num_two_count * num_three_count
    );

    Ok(())
}
