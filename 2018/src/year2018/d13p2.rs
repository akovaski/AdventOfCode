use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use super::d13p1::{self, Cart, Point, Track, Turn};

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day13.txt")?;
    let reader = BufReader::new(f);

    let mut map: BTreeMap<Point, Track> = BTreeMap::new();
    let mut carts: BTreeMap<Point, Cart> = BTreeMap::new();

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            let map_char = match c {
                '|' | 'v' | '^' => Some(Track::Vertical),
                '-' | '<' | '>' => Some(Track::Horizontal),
                '/' => Some(Track::DiagonalUp),
                '\\' => Some(Track::DiagonalDown),
                '+' => Some(Track::Intersect),
                ' ' => None,
                _ => {
                    println!("unrecognized map char: {}", c);
                    panic!();
                }
            };
            let p = Point { x, y };

            if let Some(map_char) = map_char {
                map.insert(p, map_char);
            }

            match c {
                'v' => {
                    carts.insert(p, Cart::Down(Turn::Left));
                }
                '^' => {
                    carts.insert(p, Cart::Up(Turn::Left));
                }
                '>' => {
                    carts.insert(p, Cart::Right(Turn::Left));
                }
                '<' => {
                    carts.insert(p, Cart::Left(Turn::Left));
                }
                _ => {}
            }
        }
    }

    loop {
        d13p1::simulate_one_tick(&map, &mut carts);

        carts = carts
            .iter()
            .filter_map(|(&p, &c)| match c {
                Cart::Crash => None,
                _ => Some((p, c)),
            })
            .collect();

        if carts.len() <= 1 {
            break;
        }
    }

    let last_point: Point = *carts.keys().next().unwrap();
    println!("{},{}", last_point.x, last_point.y);

    Ok(())
}
