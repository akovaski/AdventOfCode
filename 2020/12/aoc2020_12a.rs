use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let directions = parse_directions(&lines);

    let mut x = 0;
    let mut y = 0;
    let mut face = CardinalDir::E;
    for dir in directions {
        match dir {
            NavAction::North(amount) => y += amount,
            NavAction::East(amount) => x += amount,
            NavAction::South(amount) => y -= amount,
            NavAction::West(amount) => x -= amount,
            NavAction::Forward(amount) => match face {
                CardinalDir::N => y += amount,
                CardinalDir::E => x += amount,
                CardinalDir::S => y -= amount,
                CardinalDir::W => x -= amount,
            },
            NavAction::Right(rot) => face = rotate_face_right(face, rot),
            NavAction::Left(rot) => face = rotate_face_left(face, rot),
        }
    }

    println!("Manhattan distance: {}", x.abs() + y.abs());

    Ok(())
}

#[derive(Debug)]
enum CardinalDir {
    N, // North
    E, // East
    S, // South
    W, // West
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
fn rotate_face_right(face: CardinalDir, rot: Rotation) -> CardinalDir {
    match rot {
        Rotation::_90 => match face {
            CardinalDir::N => CardinalDir::E,
            CardinalDir::E => CardinalDir::S,
            CardinalDir::S => CardinalDir::W,
            CardinalDir::W => CardinalDir::N,
        },
        Rotation::_180 => rotate_face_right(rotate_face_right(face, Rotation::_90), Rotation::_90),
        Rotation::_270 => rotate_face_right(rotate_face_right(face, Rotation::_180), Rotation::_90),
    }
}
fn rotate_face_left(face: CardinalDir, rot: Rotation) -> CardinalDir {
    match rot {
        Rotation::_90 => rotate_face_right(face, Rotation::_270),
        Rotation::_180 => rotate_face_right(face, Rotation::_180),
        Rotation::_270 => rotate_face_right(face, Rotation::_90),
    }
}
