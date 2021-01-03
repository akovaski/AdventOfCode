use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    assert!(lines.len() == 1);
    let mut cups: VecDeque<u8> = lines[0].chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

    for _ in 0..10_000_000 {
        simulate_round(&mut cups);
    }

    let one_pos = cups.iter().position(|c| *c == 1).unwrap();
    cups.rotate_left(one_pos);
    let circle_label = cups.iter().skip(1).map(|c| c.to_string()).fold(String::new(), |a, b| a + &b);

    println!("Cup circle label: {}", circle_label);

    Ok(())
}

fn simulate_round(cups: &mut VecDeque<u8>) {
    let current = cups.pop_front().unwrap();
    let a = cups.pop_front().unwrap();
    let b = cups.pop_front().unwrap();
    let c = cups.pop_front().unwrap();

    cups.push_back(current);

    fn wrap(s: u8) -> u8 {
        ((s + 8) % 9) + 1
    }
    let mut search = current;
    loop {
        search = wrap(search - 1);
        if let Some(position) = cups.iter().position(|c| *c == search) {
            cups.insert(position+1, a);
            cups.insert(position+2, b);
            cups.insert(position+3, c);
            break;
        }
    }
}
