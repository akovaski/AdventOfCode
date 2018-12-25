use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

use super::d16p1::{self, Inst, OpCode, Regs, Sample, OPCODES};

pub fn main() -> io::Result<()> {
    let (samples, test_program): (Vec<Sample>, Vec<Inst>) = d16p1::read_input()?;

    let mut possible_op_codes: Vec<HashSet<OpCode>> =
        vec![HashSet::from_iter(OPCODES.iter().map(|&o| o)); 16];

    for sample in samples {
        let before = sample.before;
        let inst = sample.inst;
        let after = sample.after;

        for op_code in OPCODES.iter() {
            let out_regs = d16p1::exec_inst(
                *op_code,
                inst.in_reg_a,
                inst.in_reg_b,
                inst.out_reg_c,
                &before,
            );

            if out_regs != after {
                let taken_op_code: Option<OpCode> = {
                    let possible = &mut possible_op_codes[inst.op_code];
                    possible.remove(op_code);
                    if possible.len() == 1 {
                        Some(*possible.iter().next().unwrap())
                    } else {
                        None
                    }
                };
                if let Some(taken_op_code) = taken_op_code {
                    remove_taken_op_code(&mut possible_op_codes, taken_op_code);
                }
            }
        }
    }

    let possible_op_codes = possible_op_codes; // no longer mutable

    // Verify that the samples did indeed make a 1-to-1 mapping of op codes
    for ops in &possible_op_codes {
        assert!(ops.len() == 1);
    }

    let num_to_op_codes: Vec<OpCode> = possible_op_codes
        .iter()
        .map(|ops| *ops.iter().next().unwrap())
        .collect();

    let mut regs: Regs = [0; 4];
    for inst in &test_program {
        let op_code = num_to_op_codes[inst.op_code];
        let in_reg_a = inst.in_reg_a;
        let in_reg_b = inst.in_reg_b;
        let out_reg_c = inst.out_reg_c;
        regs = d16p1::exec_inst(op_code, in_reg_a, in_reg_b, out_reg_c, &regs);
    }

    println!("{}", regs[0]);

    Ok(())
}

fn remove_taken_op_code(possible_op_codes: &mut Vec<HashSet<OpCode>>, taken: OpCode) {
    for i in 0..possible_op_codes.len() {
        let possible = &mut possible_op_codes[i];
        if possible.len() > 1 {
            possible.remove(&taken);

            if possible.len() == 0 {
                let new_taken = *possible.iter().next().unwrap();

                remove_taken_op_code(possible_op_codes, new_taken);
            }
        }
    }
}
