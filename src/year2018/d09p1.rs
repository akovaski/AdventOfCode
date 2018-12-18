use regex::Regex;
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

    let mut current_player = 0;
    let mut current_marble_pos = 0;

    let mut circle: Vec<usize> = Vec::new();
    let mut scores = vec![0; num_players];

    circle.push(0);

    for new_marble in 1..=last_marble {
        if new_marble % 23 == 0 {
            let new_marble_pos = (current_marble_pos + circle.len() - 7) % circle.len();
            current_marble_pos = new_marble_pos;

            let removed = circle.remove(new_marble_pos);
            scores[current_player] += new_marble + removed;
        } else {
            let new_marble_pos = (current_marble_pos + 2) % circle.len();
            current_marble_pos = new_marble_pos;

            if new_marble_pos == 0 {
                circle.push(new_marble);
                current_marble_pos = circle.len() - 1;
            } else {
                circle.insert(new_marble_pos, new_marble);
            }
        }

        current_player = if current_player + 1 == num_players {
            0
        } else {
            current_player + 1
        };
    }

    println!("{}", scores.iter().max().unwrap());
    Ok(())
}
