use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

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

    let mut id = 1;
    for coord in coord_vec {
        chizu[coord_to_lin(coord, *max_x)] = id;
        id += 1;
    }

    let mut counts = vec![0; id - 1];

    //print_map(&chizu, *max_x, *max_y);

    chizu = find_solve(&chizu, *max_x, *max_y);

    //print_map(&chizu, *max_x, *max_y);

    for &id in &chizu {
        if id == 0 {
            continue;
        }
        counts[id - 1] += 1;
    }

    for &y in &[0, *max_y] {
        for x in 0..=*max_x {
            let infinite = chizu[coord_to_lin(Coord { x, y }, *max_x)];
            if infinite != 0 {
                counts[infinite - 1] = 0;
            }
        }
    }

    for &x in &[0, *max_x] {
        for y in 0..=*max_y {
            let infinite = chizu[coord_to_lin(Coord { x, y }, *max_x)];
            if infinite != 0 {
                counts[infinite - 1] = 0;
            }
        }
    }

    println!("{}", counts.iter().max().unwrap());
    Ok(())
}

#[allow(dead_code)]
fn print_map(chizu: &Vec<usize>, max_x: usize, max_y: usize) {
    let id_map: Vec<char> = ".ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();

    println!();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", id_map[chizu[coord_to_lin(Coord { x, y }, max_x)]]);
        }
        println!();
    }
}

fn find_solve(chizu: &Vec<usize>, max_x: usize, max_y: usize) -> Vec<usize> {
    let mut coords_out = vec![0; chizu.len()];

    for y in 0..=max_y {
        for x in 0..=max_x {
            let index = coord_to_lin(Coord { x, y }, max_x);
            if coords_out[index] != 0 {
                continue;
            }

            for radius in 0..=max_x + max_y {
                let result = search_radius(chizu, max_x, max_y, x, y, radius);
                match result {
                    SearchResult::None => {}
                    SearchResult::Multiple => {
                        //println!("{},{}: found multiple", x, y);
                        break;
                    }
                    SearchResult::One(coord) => {
                        let id = chizu[coord_to_lin(coord, max_x)];
                        //println!("{},{}: found {} @ {},{}", x, y, id, coord.x, coord.y);
                        for mark_y in auto_range(y, coord.y) {
                            for mark_x in auto_range(x, coord.x) {
                                coords_out[coord_to_lin(
                                    Coord {
                                        x: mark_x,
                                        y: mark_y,
                                    },
                                    max_x,
                                )] = id;
                            }
                        }
                        break;
                    }
                };
            }
        }
    }

    coords_out
}

fn auto_range(a: usize, b: usize) -> std::ops::RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

enum SearchResult {
    None,
    One(Coord),
    Multiple,
}

fn search_radius(
    chizu: &Vec<usize>,
    max_x: usize,
    max_y: usize,
    x: usize,
    y: usize,
    radius: usize,
) -> SearchResult {
    let radius = radius as i32;
    let mut found = HashSet::new();
    for x_mod in &[1, -1] {
        for y_mod in &[1, -1] {
            for hangulation in 0..=radius {
                let search_x_int = x as i32 + x_mod * hangulation;
                let search_y_int = y as i32 + y_mod * (radius - hangulation);

                if search_x_int < 0
                    || search_x_int > max_x as i32
                    || search_y_int < 0
                    || search_y_int > max_y as i32
                {
                    continue;
                }

                let search_x = search_x_int as usize;
                let search_y = search_y_int as usize;

                let id = chizu[coord_to_lin(
                    Coord {
                        x: search_x,
                        y: search_y,
                    },
                    max_x,
                )];
                if id != 0 {
                    found.insert(Coord {
                        x: search_x,
                        y: search_y,
                    });
                }
            }
        }
    }
    match found.len() {
        0 => SearchResult::None,
        1 => SearchResult::One(*found.iter().next().unwrap()),
        _ => SearchResult::Multiple,
    }
}

#[allow(dead_code)]
fn grow_solve(chizu: Vec<usize>, max_x: usize, max_y: usize) -> Vec<usize> {
    let mut coords_out = chizu.clone();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if chizu[coord_to_lin(Coord { x, y }, max_x)] != 0 {
                continue;
            }

            let north = if y == 0 {
                0
            } else {
                chizu[coord_to_lin(Coord { x: x, y: y - 1 }, max_x)]
            };
            let south = if y == max_y {
                0
            } else {
                chizu[coord_to_lin(Coord { x: x, y: y + 1 }, max_x)]
            };
            let west = if x == 0 {
                0
            } else {
                chizu[coord_to_lin(Coord { x: x - 1, y: y }, max_x)]
            };
            let east = if x == max_x {
                0
            } else {
                chizu[coord_to_lin(Coord { x: x + 1, y: y }, max_x)]
            };

            if north != 0 && south == 0 && west == 0 && east == 0 {
                coords_out[coord_to_lin(Coord { x, y }, max_x)] = north;
            } else if north == 0 && south != 0 && west == 0 && east == 0 {
                coords_out[coord_to_lin(Coord { x, y }, max_x)] = south;
            } else if north == 0 && south == 0 && west != 0 && east == 0 {
                coords_out[coord_to_lin(Coord { x, y }, max_x)] = west;
            } else if north == 0 && south == 0 && west == 0 && east != 0 {
                coords_out[coord_to_lin(Coord { x, y }, max_x)] = east;
            }
        }
    }

    if chizu == coords_out {
        coords_out
    } else {
        grow_solve(coords_out, max_x, max_y)
    }
}

fn coord_to_lin(coord: Coord, max_x: usize) -> usize {
    (coord.y) * (max_x + 1) + coord.x
}
