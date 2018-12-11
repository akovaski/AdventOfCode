use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day07.txt")?;
    let mut reader = BufReader::new(f);

    let re = Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
    let req_vec: Vec<_> = reader
        .by_ref()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let line_ref = line.trim().as_ref();
            let cap = re.captures(line_ref).unwrap();

            let requirement: char = cap[1].parse().unwrap();
            let step: char = cap[2].parse().unwrap();

            (requirement, step)
        })
        .collect();

    let mut reqs_unfulfilled = HashMap::new();
    let mut dep_map = HashMap::new();

    for (requirement, step) in req_vec {
        let ru = reqs_unfulfilled.entry(step).or_insert(0);
        *ru += 1;

        reqs_unfulfilled.entry(requirement).or_insert(0);

        let map = dep_map.entry(requirement).or_insert(HashSet::new());
        map.insert(step);
    }

    let mut at_zero: BinaryHeap<Reverse<char>> = reqs_unfulfilled
        .iter()
        .filter(|&(_, &unful)| unful == 0)
        .map(|(&step, _)| Reverse(step))
        .collect();

    let mut output = Vec::new();

    loop {
        if let Some(next_req) = at_zero.pop() {
            let next_req = next_req.0;
            output.push(next_req);
            if let Some(deps) = dep_map.remove(&next_req) {
                for dep in deps {
                    let ru = reqs_unfulfilled.get_mut(&dep).unwrap();
                    *ru -= 1;
                    if *ru == 0 {
                        at_zero.push(Reverse(dep));
                    }
                }
            }
        } else {
            break;
        }
    }

    let output_str: String = output.iter().collect();
    println!("{}", output_str);
    Ok(())
}
