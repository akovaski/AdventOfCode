use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    assert!(lines.len() == 1);
    let numbers: Vec<i64> = lines[0].split(',').map(|ns| ns.parse().unwrap()).collect();

    let mut record: HashMap<i64, usize> = HashMap::new();
    for (i, &n) in numbers[0..numbers.len()-1].iter().enumerate() {
        record.insert(n, i);
    }

    let mut last_value = numbers[numbers.len()-1];

    for i in numbers.len()-1..30000000-1 {
        let new_value = if let Some(&last_idx) = record.get(&last_value) {
            (i - last_idx) as i64
        } else {
            0
        };
        record.insert(last_value, i);
        last_value = new_value;
    }

    println!("2020th spoken number: {}", last_value);

    Ok(())
}
