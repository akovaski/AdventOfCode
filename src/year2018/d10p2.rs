use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use super::d10p1::{self, Light, Vector};

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day10.txt")?;
    let mut reader = BufReader::new(f);

    let re =
        Regex::new(r"^position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>$").unwrap();
    let lights: Vec<_> = reader
        .by_ref()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let line_ref = line.trim().as_ref();
            let cap = re.captures(line_ref).unwrap();

            let x_pos: i32 = cap[1].parse().unwrap();
            let y_pos: i32 = cap[2].parse().unwrap();
            let x_vel: i32 = cap[3].parse().unwrap();
            let y_vel: i32 = cap[4].parse().unwrap();

            Light {
                pos: Vector { x: x_pos, y: y_pos },
                vel: Vector { x: x_vel, y: y_vel },
            }
        })
        .collect();

    let mut prev_dev = VecDeque::new();

    for t in 0.. {
        let sim = d10p1::simulate_movement(&lights, t);
        let y_avg: i32 = sim.iter().map(|l| l.pos.y).sum::<i32>() / sim.len() as i32;
        let y_dev = sim
            .iter()
            .map(|l| {
                let dev = y_avg - l.pos.y;
                dev.abs()
            })
            .sum::<i32>();

        if prev_dev.len() > 6 {
            prev_dev.pop_front();
            let rolling_dev =
                prev_dev.iter().map(|av: &(i32, i32)| av.1).sum::<i32>() / prev_dev.len() as i32;
            if y_dev > rolling_dev {
                break;
            }
        }
        prev_dev.push_back((t, y_dev));
    }

    let min_dev = prev_dev.iter().min_by_key(|(_, dev)| dev).unwrap();
    println!("{}", min_dev.0);

    Ok(())
}
