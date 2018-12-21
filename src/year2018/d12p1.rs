use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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

    let mut new_state = init_state;

    for _ in 0..20 {
        new_state = grow_generation(&new_state, &rules);
    }

    println!(
        "{}",
        new_state
            .iter()
            .filter(|&&c| c.1 == '#')
            .map(|c| c.0)
            .sum::<i32>()
    );

    Ok(())
}

fn grow_generation(state: &Vec<(i32, char)>, rules: &HashMap<String, char>) -> Vec<(i32, char)> {
    let mut new_gen = Vec::new();

    let mut stage = VecDeque::from(vec!['.'; 5]);

    let mut first_last_plant: Option<(usize, usize)> = None;

    for i in 0..state.len() + 4 {
        let c = if i < state.len() { state[i].1 } else { '.' };

        stage.pop_front().unwrap();
        stage.push_back(c);
        let ss: String = stage.iter().collect();
        let result: char = *rules.get(&ss).unwrap_or(&'.');

        if result == '#' {
            first_last_plant = Some(if let Some(fl) = first_last_plant {
                (fl.0, i)
            } else {
                (i, i)
            });
        }
        new_gen.push((state[0].0 + i as i32 - 2, result));
    }

    let range = if let Some(fl) = first_last_plant {
        fl.0..fl.1 + 1
    } else {
        0..0
    };

    new_gen[range].to_vec()
}
