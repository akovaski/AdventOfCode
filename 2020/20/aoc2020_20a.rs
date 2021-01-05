use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let tiles = parse_input(&lines);

    let mut edge_map: HashMap<i32, Vec<i64>> = HashMap::new();
    for tile in &tiles {
        for &edge_id in &tile.edge_ids {
            let list = edge_map.entry(edge_id).or_insert(Vec::new());
            list.push(tile.id);
        }
    }
    let edge_map = edge_map;

    let magic_number: i64 = tiles.iter()
        .filter(|t| t.edge_ids.iter().filter(|e| edge_map.get(e).unwrap().len() == 1).count() == 2)
        .map(|t| t.id)
        .product();

    println!("magic number {}", magic_number);
    Ok(())
}


#[derive(Debug,Clone)]
struct Tile {
    id: i64,
    edge_ids: Vec<i32>,
}

fn parse_input(lines: &[String]) -> Vec<Tile> {
    enum ParseMode {
        NewTile,
        TileLine,
    }

    let mut parse_mode = ParseMode::NewTile;
    let mut tiles = Vec::new();
    let mut current_id: i64 = 0;
    let mut current_grid: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        match parse_mode {
            ParseMode::NewTile => {
                assert!(line.len() == 10);
                current_id = line[5..9].parse().unwrap();
                current_grid = Vec::new();
                parse_mode = ParseMode::TileLine;
            }
            ParseMode::TileLine => {
                if line == "" {
                    // generate an ID for each side
                    // tiles can be flipped, so direction doesn't matter
                    let (mut top_a, mut top_b) = (0, 0);
                    let (mut right_a, mut right_b) = (0, 0);
                    let (mut bottom_a, mut bottom_b) = (0, 0);
                    let (mut left_a, mut left_b) = (0, 0);
                    for i in 0..10 {
                        top_a    = top_a    << 1 | current_grid[0][  i] as i32;
                        top_b    = top_b    << 1 | current_grid[0][9-i] as i32;
                        right_a  = right_a  << 1 | current_grid[  i][9] as i32;
                        right_b  = right_b  << 1 | current_grid[9-i][9] as i32;
                        bottom_a = bottom_a << 1 | current_grid[9][  i] as i32;
                        bottom_b = bottom_b << 1 | current_grid[9][9-i] as i32;
                        left_a   = left_a   << 1 | current_grid[  i][0] as i32;
                        left_b   = left_b   << 1 | current_grid[9-i][0] as i32;
                    }
                    let top = std::cmp::min(top_a, top_b);
                    let right = std::cmp::min(right_a, right_b);
                    let bottom = std::cmp::min(bottom_a, bottom_b);
                    let left = std::cmp::min(left_a, left_b);
                    tiles.push(Tile{id: current_id, edge_ids: vec![top, right, bottom, left]});
                    parse_mode = ParseMode::NewTile;
                } else {
                    assert!(line.len() == 10);
                    let grid_line: Vec<bool> = line.chars().map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    }).collect();
                    current_grid.push(grid_line);
                }
            }
        }
    }
    tiles
}
