use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines = BufReader::new(File::open("input.txt")?).lines();
    let mut line_numbers: Vec<i64> = lines.map(|l| l.unwrap().parse::<i64>().unwrap()).collect();

    line_numbers.sort();

    let mut cache = HashMap::new();
    println!("magic number: {}", count_perms(0, &line_numbers, &mut cache));

    Ok(())
}

fn count_perms(base: i64, nums: &[i64], cache: &mut HashMap<i64, i64>) -> i64 {
    if nums.len() <= 1 {
        1
    } else if let Some(&precalc) = cache.get(&base) {
        precalc
    } else {
        let mut count = 0;
        for (i, &next) in nums.iter().enumerate() {
            if next > base + 3 {
                break;
            }
            count += count_perms(next, &nums[i+1..], cache);
        }
        cache.insert(base, count);
        count
    }
}
