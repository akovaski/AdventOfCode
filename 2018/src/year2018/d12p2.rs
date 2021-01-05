use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use super::d12p1;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day12.txt")?;
    let mut reader = BufReader::new(f);
    let mut buf = String::new();

    let re_initial_state = Regex::new(r"^initial state: ([\.#]+)$").unwrap();
    let re_rule = Regex::new(r"^([\.#]{5}) => ([\.#])$").unwrap();

    reader.read_line(&mut buf)?;
    let cap = re_initial_state.captures(buf.trim()).unwrap();

    let init_state: Vec<(i32, char)> = cap[1]
        .chars()
        .enumerate()
        .map(|c| (c.0 as i32, c.1))
        .collect();

    reader.read_line(&mut buf)?;

    let mut rules: HashMap<String, char> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let cap = re_rule.captures(&line).unwrap();
        let lhs: String = cap[1].parse().unwrap();
        let rhs: char = cap[2].parse().unwrap();

        rules.insert(lhs, rhs);
    }

    let mut state_history = Vec::new();
    state_history.push(init_state);

    for i in 1..5000 {
        let new_gen = d12p1::grow_generation(state_history.last().unwrap(), &rules);
        state_history.push(new_gen);

        if let Some(dup_i) = find_duplicate_of_last(&state_history) {
            let cycle_length = (i - dup_i) as i64;
            let cycle_offset = (50000000000 - i as i64) % cycle_length as i64;
            let cycle_travel = state_history[i][0].0 - state_history[dup_i][0].0;
            let final_duplicate = dup_i + cycle_offset as usize;
            let num_cycles = (50000000000 - final_duplicate as i64) / cycle_length;

            let final_gen_sum: i64 = state_history[final_duplicate]
                .iter()
                .filter(|c| c.1 == '#')
                .map(|c| c.0 as i64 + cycle_travel as i64 * num_cycles)
                .sum();
            println!("{}", final_gen_sum);
            break;
        }
    }

    Ok(())
}

fn find_duplicate_of_last(state_history: &Vec<Vec<(i32, char)>>) -> Option<usize> {
    let last = state_history.last().unwrap();

    let mut duplicate = None;

    for i in 0..state_history.len() - 1 {
        let curr = state_history.get(i).unwrap();
        if last.len() != curr.len() {
            continue;
        }

        let mut same = true;
        for p_i in 0..last.len() {
            if last[p_i].1 != curr[p_i].1 {
                same = false;
                continue;
            }
        }

        if same {
            duplicate = Some(i);
            break;
        }
    }

    duplicate
}
