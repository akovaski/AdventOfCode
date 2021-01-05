use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let mut cube_map = parse_map(&lines);

    for _ in 0..6 {
        cube_map = simulate_round(&cube_map);
    }

    println!("active cubes: {}", cube_map.len());

    Ok(())
}

fn simulate_round(cube_map: &HashMap<Point, ()>) -> HashMap<Point, ()> {
    let mut points_to_check = cube_map.clone();
    for &point in cube_map.keys() {
        for n in neighbors(point) {
            points_to_check.insert(n, ());
        }
    }
    let points_to_check = points_to_check;

    let mut new_cube_map = HashMap::new();
    for &point in points_to_check.keys() {
        let active = cube_map.contains_key(&point);
        let point_neighbors = neighbors(point);
        let num_active_neighbors = point_neighbors.iter().filter(|p| cube_map.contains_key(p)).count();
        if num_active_neighbors == 3 || active && num_active_neighbors == 2 {
            new_cube_map.insert(point, ());
        }
    }
    new_cube_map
}

type Point = (i64, i64, i64);

fn neighbors((x, y, z): Point) -> Vec<Point> {
    let mut result = Vec::new();
    for xd in -1..=1 {
        for yd in -1..=1 {
            for zd in -1..=1 {
                if xd == 0 && yd == 0 && zd == 0 {
                    continue;
                }
                result.push((x + xd, y + yd, z + zd));
            }
        }
    }
    result
}

fn parse_map(lines: &[String]) -> HashMap<Point, ()> {
    let mut cube_map = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cube_map.insert((x as i64, y as i64, 0), ());
            }
        }
    }
    cube_map
}
