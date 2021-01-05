use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines = BufReader::new(File::open("input.txt")?).lines();
    let mut line_numbers: Vec<i64> = lines.map(|l| l.unwrap().parse::<i64>().unwrap()).collect();

    line_numbers.sort();

    let mut jolt1 = 0;
    let mut jolt3 = 0;
    let mut last_jolt = 0;

    for &jolt in &line_numbers {
        let diff = jolt - last_jolt;
        last_jolt = jolt;

        match diff {
            1 => jolt1 += 1,
            3 => jolt3 += 1,
            _ => {}
        }
    }

    jolt3 += 1; //adapter to device

    println!("magic number: {}", jolt1 * jolt3);

    Ok(())
}
