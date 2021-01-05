use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let (mut seat_map, size) = create_seat_map(&lines);

    let mut changed = true;

    while changed {
        seat_map = simulate_round(&seat_map, &mut changed, size);
    }

    println!("occupied: {}", seat_map.values().filter(|&&v| v == true).count());

    Ok(())
}

#[allow(dead_code)]
fn print_seats(seat_map: &HashMap<(i32, i32), bool>, size: (i32, i32)) {
    let (width, height) = size;
    for i in 0..height {
        for j in 0..width {
            let display_char;
            if let Some(&taken) = seat_map.get(&(i, j)) {
                if taken {
                    display_char = '#'; // taken
                } else {
                    display_char = 'L'; // avail
                }
            } else {
                display_char = '.'; // floor
            }
            print!("{}", display_char);
        }
        println!();
    }
}

fn simulate_round(seat_map: &HashMap<(i32, i32), bool>, changed: &mut bool, size: (i32, i32)) -> HashMap<(i32, i32), bool> {
    let (width, height) = size;
    let mut new_seat_map = seat_map.clone();
    *changed = false;
    for (&seat, &taken) in seat_map {
        let (i, j) = seat;
        let mut neighbors = 0;
        for idiff in -1..=1 {
            for jdiff in -1..=1 {
                if idiff == 0 && jdiff == 0 {
                    continue;
                }
                for dist in 1.. {
                    let n_i = i + idiff*dist;
                    let n_j = j + jdiff*dist;
                    if n_i < 0 || n_j < 0 || n_i >= height || n_j >= width {
                        break;
                    }
                    if let Some(&n_taken) = seat_map.get(&(n_i, n_j)) {
                        if n_taken {
                            neighbors += 1;
                        }
                        break;
                    }
                }
            }
        }
        match neighbors {
            0 => {
                *changed = *changed || taken == false;
                new_seat_map.insert(seat, true);
            },
            1..=4 => {},
            5..=8 => {
                *changed = *changed || taken == true;
                new_seat_map.insert(seat, false);
            },
            _ => unreachable!(),
        }
    }
    new_seat_map
}

fn create_seat_map(lines: &[String]) -> (HashMap<(i32, i32), bool>, (i32, i32)) {
    let mut seat_map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (i, line) in lines.iter().enumerate() {
        let i = i as i32;
        height = std::cmp::max(height, i + 1);
        for (j, c) in line.chars().enumerate() {
            let j = j as i32;
            width = std::cmp::max(width, j + 1);
            if c == 'L' {
                seat_map.insert((i, j), false);
            }
        }
    }
    (seat_map, (width, height))
}
