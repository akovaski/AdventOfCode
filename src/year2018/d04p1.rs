use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day04.txt")?;
    let mut reader = BufReader::new(f);

    let re_line = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.+)").unwrap();
    let re_time = Regex::new(r"\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})").unwrap();
    let re_begin_shift = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut ord = BTreeMap::new();
    for l in reader.by_ref().lines() {
        let line = l.unwrap();
        let line_ref = line.trim().as_ref();
        let cap = re_line.captures(line_ref).unwrap();
        ord.insert(cap[1].to_owned(), cap[2].to_owned());
    }

    let mut guard_sleep_times = HashMap::new();
    let mut start_sleep_time: Option<usize> = None;
    let mut current_id: Option<usize> = None;
    for (time, action) in ord.iter() {
        let cap_time = re_time.captures(time).unwrap();
        let time: usize = cap_time[2].parse().unwrap();

        match action.as_ref() {
            "falls asleep" => {
                start_sleep_time = Some(time);
            }
            "wakes up" => {
                let time_arr: &mut [usize; 60] = guard_sleep_times
                    .entry(current_id.unwrap())
                    .or_insert([0; 60]);
                for t in start_sleep_time.unwrap()..time {
                    time_arr[t] += 1;
                }
            }
            _ => {
                let cap_guard_id = re_begin_shift.captures(action).unwrap();
                let id: usize = cap_guard_id[1].parse().unwrap();
                current_id = Some(id);
            }
        }
    }

    let (max_guard_id, _): &(usize, usize) = &guard_sleep_times
        .iter()
        .map(|(&guard, &time_arr)| (guard, time_arr.iter().sum()))
        .max_by_key(|&(_, total_time)| total_time)
        .unwrap();

    let most_slept_time = &guard_sleep_times[&max_guard_id]
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .unwrap()
        .0;

    println!("{}", max_guard_id * most_slept_time);

    Ok(())
}
