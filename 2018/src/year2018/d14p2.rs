use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day14.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut recipes: Vec<u8> = vec![3, 7];

    let search_digits: Vec<u8> = buf.chars().map(|c| c as u8 - '0' as u8).collect();

    let mut elves = [0, 1];

    loop {
        let total_score = recipes[elves[0]] + recipes[elves[1]];

        let new_score_1 = total_score / 10;
        if new_score_1 != 0 {
            recipes.push(new_score_1);
        }

        if search_matches_end(&recipes, &search_digits) {
            break;
        }

        let new_score_2 = total_score % 10;
        recipes.push(new_score_2);

        if search_matches_end(&recipes, &search_digits) {
            break;
        }

        for elf in &mut elves {
            *elf += 1 + recipes[*elf] as usize;
            *elf %= recipes.len();
        }
    }

    println!("{}", recipes.len() - search_digits.len());

    Ok(())
}

fn search_matches_end(input: &Vec<u8>, search: &Vec<u8>) -> bool {
    if input.len() < search.len() {
        return false;
    }

    let mut matches = true;

    for i in 0..search.len() {
        if input[input.len() - search.len() + i] != search[i] {
            matches = false;
            break;
        }
    }

    matches
}
