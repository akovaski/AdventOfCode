use regex::Regex;
use std::collections::LinkedList;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day09.txt")?;
    let mut reader = BufReader::new(f);

    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let cap = re.captures(&buf).unwrap();

    let num_players: usize = cap[1].parse().unwrap();
    let last_marble: usize = cap[2].parse().unwrap();

    println!("{}", simulate_highest_score(num_players, last_marble));
    Ok(())
}

pub fn simulate_highest_score(num_players: usize, last_marble: usize) -> usize {
    let mut current_player = 0;

    let mut circle: LinkedList<usize> = LinkedList::new();
    let mut scores = vec![0; num_players];

    circle.push_back(0);

    for new_marble in 1..=last_marble {
        if new_marble % 23 == 0 {
            for _ in 0..7 {
                let popped = circle.pop_back().unwrap();
                circle.push_front(popped);
            }

            scores[current_player] += new_marble + circle.pop_back().unwrap();

            let popped = circle.pop_front().unwrap();
            circle.push_back(popped);
        } else {
            let popped = circle.pop_front().unwrap();
            circle.push_back(popped);
            circle.push_back(new_marble);
        }

        current_player = if current_player + 1 == num_players {
            0
        } else {
            current_player + 1
        };
    }

    *scores.iter().max().unwrap()
}
