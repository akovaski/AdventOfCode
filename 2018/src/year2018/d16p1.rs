use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Inst {
    pub op_code: usize,
    pub in_reg_a: usize,
    pub in_reg_b: usize,
    pub out_reg_c: usize,
}

pub type Regs = [i32; 4];

#[derive(Debug)]
pub struct Sample {
    pub before: Regs,
    pub inst: Inst,
    pub after: Regs,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OpCode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

lazy_static! {
    pub static ref OPCODES: Vec<OpCode> = vec![
        OpCode::addr,
        OpCode::addi,
        OpCode::mulr,
        OpCode::muli,
        OpCode::banr,
        OpCode::bani,
        OpCode::borr,
        OpCode::bori,
        OpCode::setr,
        OpCode::seti,
        OpCode::gtir,
        OpCode::gtri,
        OpCode::gtrr,
        OpCode::eqir,
        OpCode::eqri,
        OpCode::eqrr
    ];
}

pub fn main() -> io::Result<()> {
    let (samples, _): (Vec<Sample>, Vec<Inst>) = read_input()?;

    let mut num_with_three_or_more_faux = 0;

    for sample in samples {
        let before = sample.before;
        let inst = sample.inst;
        let after = sample.after;

        let mut num_faux = 0;

        for op_code in OPCODES.iter() {
            let out_regs = exec_inst(
                *op_code,
                inst.in_reg_a,
                inst.in_reg_b,
                inst.out_reg_c,
                &before,
            );

            if out_regs == after {
                num_faux += 1;
            }
        }

        if num_faux >= 3 {
            num_with_three_or_more_faux += 1;
        }
    }

    println!("{}", num_with_three_or_more_faux);

    Ok(())
}

pub fn exec_inst(
    op_code: OpCode,
    in_reg_a: usize,
    in_reg_b: usize,
    out_reg_c: usize,
    regs: &Regs,
) -> Regs {
    let val_a = in_reg_a as i32;
    let val_b = in_reg_b as i32;
    let reg_a = regs[in_reg_a];
    let reg_b = regs[in_reg_b];

    let out_val = match op_code {
        OpCode::addr => reg_a + reg_b,
        OpCode::addi => reg_a + val_b,
        OpCode::mulr => reg_a * reg_b,
        OpCode::muli => reg_a * val_b,
        OpCode::banr => reg_a & reg_b,
        OpCode::bani => reg_a & val_b,
        OpCode::borr => reg_a | reg_b,
        OpCode::bori => reg_a | val_b,
        OpCode::setr => reg_a,
        OpCode::seti => val_a,
        OpCode::gtir => {
            if val_a > reg_b {
                1
            } else {
                0
            }
        }
        OpCode::gtri => {
            if reg_a > val_b {
                1
            } else {
                0
            }
        }
        OpCode::gtrr => {
            if reg_a > reg_b {
                1
            } else {
                0
            }
        }
        OpCode::eqir => {
            if val_a == reg_b {
                1
            } else {
                0
            }
        }
        OpCode::eqri => {
            if reg_a == val_b {
                1
            } else {
                0
            }
        }
        OpCode::eqrr => {
            if reg_a == reg_b {
                1
            } else {
                0
            }
        }
    };

    let mut output_regs = regs.clone();
    output_regs[out_reg_c] = out_val;
    output_regs
}

// parses the sample inputs and the test program
pub fn read_input() -> io::Result<(Vec<Sample>, Vec<Inst>)> {
    let f = File::open("./input/day16.txt")?;
    let mut reader = BufReader::new(f);

    let re_before = Regex::new(r"^Before: \[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
    let re_inst = Regex::new(r"^(\d+) (\d) (\d) (\d)$").unwrap();
    let re_after = Regex::new(r"^After:  \[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();

    // read samples
    let mut samples: Vec<Sample> = Vec::new();
    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        if buf.trim().len() > 0 {
            let before_cap = re_before.captures(buf.trim()).unwrap();
            let before_regs = parse_regs(&before_cap);

            buf = String::new();
            reader.read_line(&mut buf)?;
            let inst_cap = re_inst.captures(buf.trim()).unwrap();
            let inst = parse_inst(&inst_cap);

            buf = String::new();
            reader.read_line(&mut buf)?;
            let after_cap = re_after.captures(buf.trim()).unwrap();
            let after_regs = parse_regs(&after_cap);

            buf = String::new();
            reader.read_line(&mut buf)?;
            assert!(buf.trim().len() == 0); // blank line

            samples.push(Sample {
                before: before_regs,
                inst: inst,
                after: after_regs,
            });
        } else {
            break;
        }
    }

    {
        // consume one blank line
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        assert!(buf.trim().len() == 0);
    }

    let mut test_program: Vec<Inst> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let inst_cap = re_inst.captures(&line).unwrap();
        let inst = parse_inst(&inst_cap);
        test_program.push(inst);
    }

    Ok((samples, test_program))
}

fn parse_regs(cap: &regex::Captures) -> Regs {
    [
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].parse().unwrap(),
        cap[4].parse().unwrap(),
    ]
}

fn parse_inst(cap: &regex::Captures) -> Inst {
    Inst {
        op_code: cap[1].parse().unwrap(),
        in_reg_a: cap[2].parse().unwrap(),
        in_reg_b: cap[3].parse().unwrap(),
        out_reg_c: cap[4].parse().unwrap(),
    }
}
