use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let instructions = parse_instructions(&lines);

    let mut memory = HashMap::new();
    let mut current_mask = (0, Vec::new());

    for inst in instructions {
        match inst {
            Instr::Mask(ones_mask, x_mask) => current_mask = (ones_mask, x_mask.clone()),
            Instr::Mem(addr, val) => {
                let (ones_mask, x_mask) = &current_mask;
                let adjusted_addr = addr | ones_mask;
                for perm_addr in permute_address(adjusted_addr, &x_mask) {
                    memory.insert(perm_addr, val);
                }
            }
        }
    }

    println!("magic number: {}", memory.values().sum::<i64>());

    Ok(())
}

fn permute_address(addr: i64, x_mask: &[usize]) -> Vec<i64> {
    if x_mask.len() == 0 {
        return vec![addr];
    }
    let idx = x_mask[0];
    let other_addresses = permute_address(addr, &x_mask[1..]);
    let mut addresses = other_addresses.clone();
    for other in other_addresses {
        addresses.push(other ^ (1 << idx));
    }
    addresses
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
                let mut mask_string = mask_string.chars().collect::<Vec<char>>();
                mask_string.reverse();
                let x_mask = mask_string.iter().enumerate()
                    .filter(|(_,&c)| c == 'X')
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>();
                Instr::Mask(ones_mask, x_mask)
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
    Mask(i64, Vec<usize>), // ones, zeroes
    Mem(i64, i64), // addr, value
}
