use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

const MAX_LIMIT: usize = 10000;

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day06.txt")?;
    let mut reader = BufReader::new(f);

    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    let coord_vec: Vec<_> = reader
        .by_ref()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let line_ref = line.trim().as_ref();
            let cap = re.captures(line_ref).unwrap();
            Coord {
                x: cap[1].parse::<usize>().unwrap() - 1,
                y: cap[2].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    let (max_x, max_y) = &coord_vec.iter().fold((0, 0), |acc, coord| {
        (cmp::max(acc.0, coord.x), cmp::max(acc.1, coord.y))
    });

    let mut chizu = vec![0; (max_x + 1) * (max_y + 1)];

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            let pos = coord_to_lin(Coord { x, y }, *max_x);
            if chizu[pos] > 0 {
                continue;
            }

            let total_distance = get_total_distance(Coord { x, y }, &coord_vec);

            do_mark_radius(&mut chizu, Coord { x, y }, total_distance, *max_x, *max_y, coord_vec.len());
        }
    }
    
    //print_map(&chizu, *max_x, *max_y);
    let solution: usize = chizu.iter().map(|&val| if val < MAX_LIMIT { 1 } else { 0 }).sum();

    println!("{}", solution);

    Ok(())
}

fn do_mark_radius(
    chizu: &mut Vec<usize>,
    coord: Coord,
    total_distance: usize,
    max_x: usize,
    max_y: usize,
    vec_size: usize,
) {
    let mark_radius = if total_distance >= MAX_LIMIT {
        total_distance - MAX_LIMIT
    } else {
        MAX_LIMIT - total_distance - 1
    } / vec_size;

    for r in 0..=mark_radius {
        for x_mod in &[1, -1] {
            for y_mod in &[1, -1] {
                let r = r as i32;
                for hangulation in 0..=r {
                    let search_x_int = coord.x as i32 + x_mod * hangulation;
                    let search_y_int = coord.y as i32 + y_mod * (r - hangulation);

                    if search_x_int < 0
                        || search_x_int > max_x as i32
                        || search_y_int < 0
                        || search_y_int > max_y as i32
                    {
                        continue;
                    }

                    let search_x = search_x_int as usize;
                    let search_y = search_y_int as usize;

                    chizu[coord_to_lin(
                        Coord {
                            x: search_x,
                            y: search_y,
                        },
                        max_x,
                    )] = if total_distance >= MAX_LIMIT {
                        total_distance - r as usize
                    } else {
                        total_distance + r as usize
                    };
                }
            }
        }
    }
}

fn get_total_distance(coord: Coord, coord_vec: &Vec<Coord>) -> usize {
    coord_vec.iter().fold(0, |total_distance, ct| {
        total_distance + cmp::max(coord.x, ct.x) - cmp::min(coord.x, ct.x) + cmp::max(coord.y, ct.y)
            - cmp::min(coord.y, ct.y)
    })
}

#[allow(dead_code)]
fn print_map(chizu: &Vec<usize>, max_x: usize, max_y: usize) {

    println!();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{:02} ", chizu[coord_to_lin(Coord { x, y }, max_x)]);
        }
        println!();
    }
}

fn coord_to_lin(coord: Coord, max_x: usize) -> usize {
    (coord.y) * (max_x + 1) + coord.x
}
