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
    let mut req_map = HashMap::new();

    for (requirement, step) in req_vec {
        let ru = reqs_unfulfilled.entry(step).or_insert(0);
        *ru += 1;

        reqs_unfulfilled.entry(requirement).or_insert(0);

        let map = dep_map.entry(requirement).or_insert(HashSet::new());
        map.insert(step);

        let map = req_map.entry(step).or_insert(HashSet::new());
        map.insert(requirement);
    }

    let mut at_zero: BinaryHeap<Reverse<char>> = reqs_unfulfilled
        .iter()
        .filter(|&(_, &unful)| unful == 0)
        .map(|(&step, _)| Reverse(step))
        .collect();

    let mut workers = [(0, ' '); 5];
    let static_time = 60;

    let mut total_work = 0;

    loop {
        if let Some(next_req) = at_zero.pop() {
            let next_req = next_req.0;

            // get worker with least number of seconds left, including workers with no work
            let (id, &work) = workers
                .iter()
                .enumerate()
                .min_by_key(|(_, &w)| w.0)
                .unwrap();

            // free up work for at least one worker
            do_work(
                &mut workers,
                id,
                &mut dep_map,
                &mut reqs_unfulfilled,
                &mut at_zero,
            );

            total_work += work.0;

            // give the worker work
            workers[id].0 = static_time + next_req as usize - 'A' as usize + 1;
            workers[id].1 = next_req;
        } else if workers.iter().any(|&w| w.0 > 0) {
            // get worker with least number of seconds left, ignoring workers with no work
            let (id, &work) = workers
                .iter()
                .enumerate()
                .filter(|(_, w)| w.0 != 0)
                .min_by_key(|(_, &w)| w.0)
                .unwrap();

            // free up work for the next worker
            do_work(
                &mut workers,
                id,
                &mut dep_map,
                &mut reqs_unfulfilled,
                &mut at_zero,
            );

            total_work += work.0;
        } else if dep_map.len() > 0 {
            panic!("Dep map should be cleared after all workers are done working");
        } else {
            break;
        }
    }

    println!("{}", total_work);

    Ok(())
}

fn do_work(
    workers: &mut [(usize, char); 5],
    worker_id: usize,
    dep_map: &mut HashMap<char, HashSet<char>>,
    reqs_unfulfilled: &mut HashMap<char, usize>,
    at_zero: &mut BinaryHeap<Reverse<char>>,
) {
    let work = workers[worker_id].0;
    for w in workers.iter_mut() {
        w.0 -= std::cmp::min(work, w.0); // decrement the work, but don't drop past 0.

        // just counted down to 0, lower restrains on dependencies
        if work != 0 && w.0 == 0 {
            if let Some(deps) = dep_map.remove(&w.1) {
                for dep in deps {
                    let ru = reqs_unfulfilled.get_mut(&dep).unwrap();
                    *ru -= 1;
                    if *ru == 0 {
                        at_zero.push(Reverse(dep));
                    }
                }
            }
        }
    }
}
