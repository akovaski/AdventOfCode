use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let directions = parse_directions(&lines);

    let mut x = 0;
    let mut y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    for dir in directions {
        match dir {
            NavAction::North(amount) => waypoint_y += amount,
            NavAction::East(amount) => waypoint_x += amount,
            NavAction::South(amount) => waypoint_y -= amount,
            NavAction::West(amount) => waypoint_x -= amount,
            NavAction::Forward(amount) => {
                x += waypoint_x * amount;
                y += waypoint_y * amount;
            },
            NavAction::Right(rot) => {
                let (nwx, nwy) = rotate_waypoint_right(waypoint_x, waypoint_y, rot);
                waypoint_x = nwx;
                waypoint_y = nwy;
            },
            NavAction::Left(rot) => {
                let (nwx, nwy) = rotate_waypoint_left(waypoint_x, waypoint_y, rot);
                waypoint_x = nwx;
                waypoint_y = nwy;
            },

        }
    }

    println!("Manhattan distance: {}", x.abs() + y.abs());

    Ok(())
}

#[derive(Debug)]
enum NavAction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Forward(i32),
    Right(Rotation),
    Left(Rotation),
}
#[derive(Debug)]
enum Rotation {
    _90,
    _180,
    _270,
}

fn parse_directions(lines: &[String]) -> Vec<NavAction> {
    let mut directions = Vec::new();
    for line in lines {
        assert!(line.len() >= 2);
        let amount = line[1..].parse::<i32>().unwrap();
        let dir = match line.chars().nth(0).unwrap() {
            'N' => NavAction::North(amount),
            'E' => NavAction::East(amount),
            'S' => NavAction::South(amount),
            'W' => NavAction::West(amount),
            'F' => NavAction::Forward(amount),
            'R' => NavAction::Right(parse_rotation(amount)),
            'L' => NavAction::Left(parse_rotation(amount)),
            _ => unreachable!(),
        };
        directions.push(dir);
    }
    directions
}
fn parse_rotation(degrees: i32) -> Rotation {
    match degrees {
        90 => Rotation::_90,
        180 => Rotation::_180,
        270 => Rotation::_270,
        _ => unreachable!(),
    }
}
fn rotate_waypoint_right(waypoint_x: i32, waypoint_y: i32, rot: Rotation) -> (i32, i32) {
    match rot {
        Rotation::_90 => (waypoint_y, -waypoint_x),
        Rotation::_180 => (-waypoint_x, -waypoint_y),
        Rotation::_270 => (-waypoint_y, waypoint_x),
    }
}
fn rotate_waypoint_left(waypoint_x: i32, waypoint_y: i32, rot: Rotation) -> (i32, i32) {
    match rot {
        Rotation::_90 => rotate_waypoint_right(waypoint_x, waypoint_y, Rotation::_270),
        Rotation::_180 => rotate_waypoint_right(waypoint_x, waypoint_y, Rotation::_180),
        Rotation::_270 => rotate_waypoint_right(waypoint_x, waypoint_y, Rotation::_90),
    }
}
