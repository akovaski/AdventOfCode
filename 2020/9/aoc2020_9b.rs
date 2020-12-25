use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let lines = BufReader::new(File::open("input.txt")?).lines();
    let line_numbers: Vec<i64> = lines.map(|l| l.unwrap().parse::<i64>().unwrap()).collect();

    let mut nums = VecDeque::new();

    let mut invalid_number = None;

    for &line_num in &line_numbers {
        if nums.len() >= 25 {
            if !contains_sum(&nums, line_num) {
                println!("Invalid number: {}", line_num);
                invalid_number = Some(line_num);
                break;
            }
            nums.pop_front();
        }
        nums.push_back(line_num);
    }

    let invalid_number = invalid_number.unwrap();

    for i in 0..line_numbers.len() {
        let mut sum = 0;
        for j in i .. line_numbers.len() {
            sum += line_numbers[j];
            if sum == invalid_number {
                let seq = &line_numbers[i ..= j];
                println!("range: {}-{}, code: {}", i, j, seq.iter().min().unwrap() + seq.iter().max().unwrap());
                return Ok(());
            } else if sum > invalid_number {
                break;
            }
        }
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
