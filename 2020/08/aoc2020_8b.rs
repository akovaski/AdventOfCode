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

    let mut executed_instructions = HashSet::new();
    let program_result = execute_program(&program, 0, 0, &mut executed_instructions);
    dbg!(program_result);
    Ok(())
}

#[derive(Debug)]
enum ProgramComplete {
    InfLoop,
    Finished,
}

fn execute_program(program: &Vec<Inst>, mut pc: usize, mut acc: i32, executed_instructions: &mut HashSet<usize>) -> ProgramComplete {
    loop {
        match execute_inst(&program, pc, acc, executed_instructions) {
            ProgramState::InfLoop => {
                println!("Infinite loop, pc: {}  acc: {}", pc, acc);
                return ProgramComplete::InfLoop;
            }
            ProgramState::Finished => {
                println!("Finished, pc: {}  acc: {}", pc, acc);
                return ProgramComplete::Finished;
            }
            ProgramState::Continue(pc_new, acc_new) => {
                pc = pc_new;
                acc = acc_new;
            }
            ProgramState::Split(split_a, split_b) => {
                let mut ei_copy = executed_instructions.clone();
                let (b_pc, b_acc) = split_b;

                match execute_program_nosplit(&program, b_pc, b_acc, &mut ei_copy) {
                    ProgramComplete::InfLoop => {}
                    ProgramComplete::Finished => {
                        return ProgramComplete::Finished;
                    }
                }
                let (a_pc, a_acc) = split_a;
                pc = a_pc;
                acc = a_acc;
            }
        }
    }
}

fn execute_program_nosplit(program: &Vec<Inst>, mut pc: usize, mut acc: i32, executed_instructions: &mut HashSet<usize>) -> ProgramComplete {
    loop {
        match execute_inst(&program, pc, acc, executed_instructions) {
            ProgramState::InfLoop => {
                println!("Infinite loop, pc: {}  acc: {}", pc, acc);
                return ProgramComplete::InfLoop;
            }
            ProgramState::Finished => {
                println!("Finished, pc: {}  acc: {}", pc, acc);
                return ProgramComplete::Finished;
            }
            ProgramState::Continue(pc_new, acc_new) => {
                pc = pc_new;
                acc = acc_new;
            }
            ProgramState::Split(split_a, _) => {
                let (a_pc, a_acc) = split_a;
                pc = a_pc;
                acc = a_acc;
            }
        }
    }
}

enum ProgramState {
    InfLoop,
    Finished,
    Continue(usize, i32), // pc, acc, executed_instructions
    Split((usize, i32), (usize, i32)),
}

fn execute_inst(program: &Vec<Inst>, mut pc: usize, mut acc: i32, executed_instructions: &mut HashSet<usize>) -> ProgramState {
    if pc == program.len() {
        return ProgramState::Finished;
    }
    if executed_instructions.contains(&pc) {
        return ProgramState::InfLoop;
    }
    executed_instructions.insert(pc);

    match program[pc] {
        Inst::Acc(x) => {
            acc += x;
            pc += 1;
        }
        Inst::Jmp(offset) => {
            return ProgramState::Split(((pc as i32 + offset) as usize, acc), (pc+1, acc));
        }
        Inst::Noop(offset) => {
            return ProgramState::Split((pc+1, acc), ((pc as i32 + offset) as usize, acc));
        }
    }

    return ProgramState::Continue(pc, acc);
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
