use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day02.txt")?;
    let mut reader = BufReader::new(f);

    let box_vec: Vec<String> = reader.by_ref().lines().map(|l| l.unwrap()).collect();

    let mut partial_boxes = HashMap::new();
    for id in box_vec {
        for (i, _) in id.chars().enumerate() {
            let mut result_id = id.clone();
            result_id.replace_range(i..i + 1, "?");

            let count = partial_boxes.entry(result_id.clone()).or_insert(0);
            *count += 1;

            if *count >= 2 {
                let mut output_str = result_id.clone();
                output_str.remove(i);
                println!("{}", output_str);
            }
        }
    }

    Ok(())
}
