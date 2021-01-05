use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

const NUM_CUPS: usize = 1_000_000;
fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    assert!(lines.len() == 1);
    let input_cups: Vec<usize> = lines[0].chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    // index is the cup, value is the next cup
    let mut next_cup: [usize; NUM_CUPS] = [0; NUM_CUPS];
    for i in 0..NUM_CUPS {
        // assign next cup
        next_cup[i] = if i < input_cups.len() {
            let cup_pos = input_cups.iter().position(|c| *c == i + 1).unwrap();
            if cup_pos < input_cups.len() - 1 {
                cup_to_i(input_cups[cup_pos + 1])
            } else {
                if input_cups.len() != NUM_CUPS {
                    input_cups.len()
                } else {
                    cup_to_i(input_cups[0])
                }
            }
        } else if i < NUM_CUPS - 1 {
            i + 1
        } else {
            cup_to_i(input_cups[0])
        };
    }

    let mut current_cup = cup_to_i(input_cups[0]);

    for _ in 0..10_000_000 {
        current_cup = simulate_round(current_cup, &mut next_cup);
    }

    let cup_a = next_cup[0];
    let cup_b = next_cup[cup_a];

    let cup_a = cup_a + 1;
    let cup_b = cup_b + 1;
    println!("Cup answer: {}", cup_a * cup_b);

    Ok(())
}


fn simulate_round(current_cup: usize, next_cup: &mut [usize]) -> usize {
    let group = [next_cup[current_cup], next_cup[next_cup[current_cup]], next_cup[next_cup[next_cup[current_cup]]]];
    let after_group = next_cup[group[2]];

    // remove group
    next_cup[current_cup] = after_group;

    // find where to insert group after
    let mut search = current_cup;
    loop {
        search = (search + NUM_CUPS - 1) % NUM_CUPS;
        if !group.contains(&search) {
            break;
        }
    }
    let after_search = next_cup[search];

    // insert group
    next_cup[search] = group[0];
    next_cup[group[2]] = after_search;

    // return next current_cup
    next_cup[current_cup]
}

#[allow(dead_code)]
fn print_label(next_cup: &[usize]) {
    let mut i = 0;
    loop {
        i = next_cup[i];
        if i == 0 {
            break;
        }
        print!("{}", i+1);
    }
    println!();
}

#[allow(dead_code)]
fn print_cup_ptr(label: &str, ptrs: &[usize]) {
    println!("\n{}", label);
    for (i, ptr) in ptrs.iter().enumerate() {
        println!("{:>3}: {:>3}", i+1, ptr+1);
    }
}

fn cup_to_i(cup: usize) -> usize {
    cup - 1
}
