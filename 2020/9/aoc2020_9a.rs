use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let lines = BufReader::new(File::open("input.txt")?).lines();

    let mut nums = VecDeque::new();

    for line in lines {
        let line_num = line?.parse::<i64>().unwrap();

        if nums.len() >= 25 {
            if !contains_sum(&nums, line_num) {
                println!("Invalid number: {}", line_num);
                return Ok(());
            }
            nums.pop_front();
        }
        nums.push_back(line_num);
    }

    Ok(())
}

fn contains_sum(nums: &VecDeque<i64>, line_num: i64) -> bool {
    for i in 0..nums.len() {
        for j in i+1 .. nums.len() {
            if nums[i] + nums[j] == line_num {
                return true;
            }
        }
    }
    return false;
}
