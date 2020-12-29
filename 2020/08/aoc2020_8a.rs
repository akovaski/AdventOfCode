use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashSet;

#[derive(Debug)]
enum Inst {
    Acc(i32),
    Jmp(i32),
    Noop(i32),
}

fn main() -> Result<()> {
    let lines = BufReader::new(File::open("input.txt")?).lines();
    let mut program = Vec::new();

    for line in lines {
        let inst = parse_line(&line?);
        program.push(inst);
    }

    let mut pc: usize = 0;
    let mut acc: i32 = 0;

    let mut executed_instructions = HashSet::new();

    loop {
        if executed_instructions.contains(&pc) {
            break;
        }
        executed_instructions.insert(pc);

        match program[pc] {
            Inst::Acc(x) => {
                acc += x;
                pc += 1;
            }
            Inst::Jmp(offset) => pc = (pc as i32 + offset) as usize,
            Inst::Noop(_) => pc += 1,
        }
    }

    println!("acc before inf-looping: {}", acc);

    Ok(())
}

fn parse_line(line: &str) -> Inst {
    let words: Vec<&str> = line.split(' ').collect();
    assert_eq!(words.len(), 2);
    let number = words[1].parse::<i32>().unwrap();

    match words[0] {
        "acc" => Inst::Acc(number),
        "jmp" => Inst::Jmp(number),
        "nop" => Inst::Noop(number),
        _ => panic!(),
    }
}
