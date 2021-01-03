use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let offsets: Vec<Vec<Step>> = lines.iter().map(|l| parse_steps(l)).collect();
    //dbg!(parse_steps(&lines[0]));
    //dbg!(Hex{q:0,r:0}.offset(&vec![Step::NorthWest, Step::West, Step::SouthWest, Step::East, Step::East]));

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

    println!("Number of black tiles: {}", tiles.len());

    Ok(())
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
