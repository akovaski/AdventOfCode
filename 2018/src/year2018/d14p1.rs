use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day14.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut recipes: Vec<u8> = vec![3, 7];

    let scores_start: usize = buf.parse().unwrap();
    let num_scores = 10;

    let mut elves = [0, 1];

    loop {
        if recipes.len() >= scores_start + num_scores {
            break;
        }

        let total_score = recipes[elves[0]] + recipes[elves[1]];

        let new_score_1 = total_score / 10;
        if new_score_1 != 0 {
            recipes.push(new_score_1);
        }

        let new_score_2 = total_score % 10;
        recipes.push(new_score_2);

        for elf in &mut elves {
            *elf += 1 + recipes[*elf] as usize;
            *elf %= recipes.len();
        }
    }

    let scores_after = &recipes[scores_start..scores_start + num_scores]
        .iter()
        .map(|s| (s + '0' as u8) as char)
        .collect::<String>();
    println!("{}", scores_after);

    Ok(())
}
