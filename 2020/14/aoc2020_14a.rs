use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let instructions = parse_instructions(&lines);

    let mut memory = HashMap::new();
    let mut current_mask = (0, 0);

    for inst in instructions {
        match inst {
            Instr::Mask(ones_mask, zeroes_mask) => current_mask = (ones_mask, zeroes_mask),
            Instr::Mem(addr, val) => {
                let (ones_mask, zeroes_mask) = current_mask;
                let adjusted_val = (val | ones_mask) & !zeroes_mask;
                memory.insert(addr, adjusted_val);
            }
        }
    }

    println!("magic number: {}", memory.values().sum::<i64>());

    Ok(())
}

fn parse_instructions(lines: &[String]) -> Vec<Instr> {
    let mut instructions = Vec::new();
    for line in lines {
        instructions.push(match &line[0..3] {
            "mas" => {
                let mask_string = line.split(" = ").nth(1).unwrap();
                let ones_mask = i64::from_str_radix(
                    &mask_string.chars().map(|c| if c == '1' { '1' } else { '0' }).collect::<String>(),
                    2).unwrap();
                let zeroes_mask = i64::from_str_radix(
                    &mask_string.chars().map(|c| if c == '0' { '1' } else { '0' }).collect::<String>(),
                    2).unwrap();
                Instr::Mask(ones_mask, zeroes_mask)
            },
            "mem" => {
                let s: Vec<&str> = line.split(" = ").collect();
                let left = s[0];
                let mem_loc = left[4 .. left.len()-1].parse::<i64>().unwrap();
                let mem_val = s[1].parse::<i64>().unwrap();
                Instr::Mem(mem_loc, mem_val)
            },
            _ => unreachable!(),
        })
    }
    instructions
}

#[derive(Debug)]
enum Instr {
    Mask(i64, i64), // ones, zeroes
    Mem(i64, i64), // addr, value
}
