use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let offsets: Vec<Vec<Step>> = lines.iter().map(|l| parse_steps(l)).collect();

    let mut tiles: HashSet<Hex> = HashSet::new();
    let base_tile = Hex { q: 0, r: 0 };
    for offset in &offsets {
        let tile = base_tile.offset(offset);
        if tiles.contains(&tile) {
            tiles.remove(&tile);
        } else {
            tiles.insert(tile);
        }
    }

    for _ in 0..100 {
        tiles = update_pattern(&tiles);
    }

    println!("Number of black tiles: {}", tiles.len());

    Ok(())
}

fn update_pattern(tiles: &HashSet<Hex>) -> HashSet<Hex> {
    let mut tiles_to_check: HashSet<Hex> = HashSet::new();
    for tile in tiles {
        tiles_to_check.insert(*tile);
        for neighbor in tile.neighbors() {
            tiles_to_check.insert(neighbor);
        }
    }

    let mut new_tiles: HashSet<Hex> = HashSet::new();
    for tile in tiles_to_check {
        let is_black = tiles.contains(&tile);
        let mut neighbor_count = 0;
        for neighbor in tile.neighbors() {
            if tiles.contains(&neighbor) {
                neighbor_count += 1;
            }
        }
        match is_black {
            true => if neighbor_count == 1 || neighbor_count == 2 {
                new_tiles.insert(tile);
            }
            false => if neighbor_count == 2 {
                new_tiles.insert(tile);
            }
        }
    }
    new_tiles
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Hex {
    q: i32,
    r: i32,
}
impl Hex {
    fn step(&self, step: Step) -> Hex {
        let (q, r) = (self.q, self.r);
        let new_pos = match step {
            Step::East => (q + 1, r),
            Step::SouthEast => (q, r + 1),
            Step::SouthWest => (q - 1, r + 1),
            Step::West => (q - 1, r),
            Step::NorthWest => (q, r - 1),
            Step::NorthEast => (q + 1, r - 1),
        };
        Hex { q: new_pos.0, r: new_pos.1 }
    }
    fn offset(&self, steps: &[Step]) -> Hex {
        let mut current: Hex = *self;
        for step in steps {
            current = current.step(*step);
        }
        current
    }
    fn neighbors(&self) -> Vec<Hex> {
        let mut n = Vec::new();
        for step in &ALL_STEPS {
            n.push(self.step(*step));
        }
        n
    }
}

#[derive(Debug,Clone,Copy)]
enum Step {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

const ALL_STEPS: [Step; 6] = [
    Step::East,
    Step::SouthEast,
    Step::SouthWest,
    Step::West,
    Step::NorthWest,
    Step::NorthEast,
];

fn parse_steps(line: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();
    while i < chars.len() {
        match chars[i] {
            'e' => {
                steps.push(Step::East);
                i += 1;
            }
            'w' => {
                steps.push(Step::West);
                i += 1;
            }
            's' => {
                match chars[i+1] {
                    'e' => steps.push(Step::SouthEast),
                    'w' => steps.push(Step::SouthWest),
                    _ => unreachable!(),
                }
                i += 2;
            }
            'n' => {
                match chars[i+1] {
                    'e' => steps.push(Step::NorthEast),
                    'w' => steps.push(Step::NorthWest),
                    _ => unreachable!(),
                }
                i += 2;
            }
            _ => unreachable!(),
        }
    }
    steps
}
