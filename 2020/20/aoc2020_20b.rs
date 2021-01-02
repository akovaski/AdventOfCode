use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;
use std::collections::VecDeque;

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

    let mut id_map: HashMap<i64, &Tile> = HashMap::new();
    for tile in &tiles {
        id_map.insert(tile.id, tile);
    }
    let id_map = id_map;

    let mut tile_map: HashMap<i64, Point> = HashMap::new(); // physical layout of tiles
    let mut loc_map: HashMap<Point, i64> = HashMap::new();
    let mut flipped_and_rotated_map: HashMap<Point, (bool, Rotation)> = HashMap::new();
    tile_map.insert(tiles[0].id, (0, 0));
    loc_map.insert((0, 0), tiles[0].id);
    flipped_and_rotated_map.insert((0, 0), (false, Rotation::_0));
    let mut edges_to_check: VecDeque<(i32, Point)> =
        generate_neighbor_requirements(&tiles[0], (false, Rotation::_0), (0,0))
        .to_vec().into();
    fn update_bounds((old_min, old_max): (i32, i32), new: i32) -> (i32, i32) {
        (std::cmp::min(old_min, new), std::cmp::max(old_max, new))
    }
    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);
    while let Some((edge_id, loc)) = edges_to_check.pop_front() {
        if let Some(list) = edge_map.get(&edge_id) {
            if list.len() == 1 {
                continue;
            }
            assert!(list.len() == 2);
            let filtered_list: Vec<i64> = list.iter().cloned().filter(|e| !tile_map.contains_key(e)).collect();
            if filtered_list.len() == 0 {
                continue;
            }
            assert!(filtered_list.len() == 1);
            let new_tile = id_map.get(&filtered_list[0]).unwrap();
            tile_map.insert(new_tile.id, loc);
            loc_map.insert(loc, new_tile.id);

            let flip_rotate = flip_and_rotate_to_match_any_neighbor(
                &id_map, &loc_map, &flipped_and_rotated_map, loc);
            flipped_and_rotated_map.insert(loc, flip_rotate);

            let (x, y) = loc;
            x_bounds = update_bounds(x_bounds, x);
            y_bounds = update_bounds(y_bounds, y);

            edges_to_check.extend(&generate_neighbor_requirements(&new_tile, flip_rotate, loc));
        }
    }

    let mut combined_tiles: Vec<Vec<bool>> = Vec::new();
    for y in y_bounds.0 ..= y_bounds.1 {
        let mut combined_8_lines = vec![vec![]; 8];
        for x in x_bounds.0 ..= x_bounds.1 {
            let tile_id = loc_map.get(&(x, y)).unwrap();
            let tile = id_map.get(tile_id).unwrap();
            for i in 0..8 {
                for j in 0..8 {
                    let (flipped, rotation) = flipped_and_rotated_map.get(&(x, y)).unwrap();
                    let inner_grid_i = match rotation {
                        Rotation::_0 => i,
                        Rotation::_90 => 7-j,
                        Rotation::_180 => 7-i,
                        Rotation::_270 => j,
                    };
                    let inner_grid_j = match flipped {
                        false => match rotation {
                            Rotation::_0 => j,
                            Rotation::_90 => i,
                            Rotation::_180 => 7-j,
                            Rotation::_270 => 7-i,
                        }
                        true => match rotation {
                            Rotation::_0 => 7-j,
                            Rotation::_90 => 7-i,
                            Rotation::_180 => j,
                            Rotation::_270 => i,
                        }
                    };

                    combined_8_lines[i].push(tile.inner_grid[inner_grid_i][inner_grid_j]);
                }
            }
        }
        combined_tiles.extend(combined_8_lines.iter().rev().cloned());
    }
    let combined_tiles = combined_tiles;

    for line in combined_tiles.iter() {
        for &val in line {
            print!("{}", if val { '#' } else { '.'});
        }
        println!();
    }

    let sea_monster =
        ["                  # ",
         "#    ##    ##    ###",
         " #  #  #  #  #  #   "]
             .iter().map(|line| line.chars().map(|c| match c {
                        ' ' => false,
                        '#' => true,
                        _ => unreachable!(),
                    }).collect::<Vec<bool>>()).collect::<Grid>();

    let num_sea_monsters = count_positive_pattern(&combined_tiles, &sea_monster);
    println!("Number of sea monsters {}", num_sea_monsters);
    
    fn count_pounds(grid: &Grid) -> i32 {
        grid.iter().map(|row| row.iter().filter(|x| **x == true).count() as i32).sum()
    }

    let image_pounds = count_pounds(&combined_tiles);
    let sea_monster_pounds = count_pounds(&sea_monster);
    println!("# count of image: {}", image_pounds);
    println!("# per sea monster: {}", sea_monster_pounds);
    println!("sea roughness: {}", image_pounds - num_sea_monsters * sea_monster_pounds);

    Ok(())
}

// TODO
fn generate_neighbor_requirements(tile: &Tile, flip_rotate: (bool, Rotation), origin: Point) -> [(i32, Point); 4] {
    let (x, y) = origin;
    let locs = [(x, y+1), (x+1, y), (x, y-1), (x-1, y)];
    let fr_e = fr_edges(&tile.edge_ids, flip_rotate);
    let mut result = [(0, (0,0)); 4];
    for i in 0..4 {
        result[i] = (normalize_edge(fr_e[i]), locs[i]);
    }
    result
}

fn normalize_edge(e: i32) -> i32 {
    std::cmp::min(e, reverse_edge(e))
}

fn count_positive_pattern(image: &Grid, pattern: &Grid) -> i32 {
    let rotations = [Rotation::_0, Rotation::_90, Rotation::_180, Rotation::_270];
    let flips = [true, false];

    let mut count = 0;
    for &rot in &rotations {
        let pattern = rotate_pattern(&pattern, rot);
        for &flip in &flips {
            let pattern = flip_pattern(&pattern, flip);

            for i in 0..image.len() {
                for j in 0..image[0].len() {
                    if test_pattern_at(&image, &pattern, (i, j)) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn test_pattern_at(image: &Grid, pattern: &Grid, idx: (usize, usize)) -> bool {
    let (i, j) = idx;
    if i + pattern.len() > image.len() || j + pattern[0].len() > image[0].len() {
        return false;
    }
    for id in 0..pattern.len() {
        for jd in 0..pattern[0].len() {
            if pattern[id][jd] && !image[i+id][j+jd] {
                return false;
            }
        }
    }
    true
}

fn rotate_pattern(pattern: &Grid, rotation: Rotation) -> Grid {
    match rotation {
        Rotation::_0 => pattern.clone(),
        Rotation::_90 => {
            let mut new_pattern = vec![vec![false; pattern.len()]; pattern[0].len()];
            for i in 0..pattern.len() {
                for j in 0..pattern[0].len() {
                    let new_i = j;
                    let new_j = pattern.len() - 1 - i;
                    new_pattern[new_i][new_j] = pattern[i][j];
                }
            }
            new_pattern
        }
        Rotation::_180 => rotate_pattern(&rotate_pattern(pattern, Rotation::_90), Rotation::_90),
        Rotation::_270 => rotate_pattern(&rotate_pattern(pattern, Rotation::_180), Rotation::_90),
    }
}

fn flip_pattern(pattern: &Grid, flip: bool) -> Grid {
    match flip {
        false => pattern.clone(),
        true => {
            let mut new_pattern = vec![vec![false; pattern[0].len()]; pattern.len()];
            for i in 0..pattern.len() {
                for j in 0..pattern[0].len() {
                    new_pattern[i][pattern[0].len() -1 -j] = pattern[i][j];
                }
            }
            new_pattern
        }
    }
}

// clockwise rotation to apply to a tilemap
#[derive(Debug,Clone,Copy)]
enum Rotation {
    _0,
    _90,
    _180,
    _270,
}

fn neighbors(loc: Point) -> Vec<Point> {
    let mut n = Vec::new();
    for d in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        n.push((loc.0 + d.0, loc.1 + d.1));
    }
    n
}

fn flip_and_rotate_to_match_any_neighbor(id_map: &HashMap<i64, &Tile>, loc_map: &HashMap<Point, i64>, flipped_and_rotated_map: &HashMap<Point, (bool, Rotation)>, loc: Point) -> (bool, Rotation) {
    let tile = id_map.get(loc_map.get(&loc).unwrap()).unwrap();
    for neighbor in &neighbors(loc) {
        if let Some(nid) = loc_map.get(neighbor) {
            let n_tile = id_map.get(nid).unwrap();
            let n_flip_rot = flipped_and_rotated_map.get(neighbor).unwrap();
            let n_edges = fr_edges(&n_tile.raw_edge_ids, *n_flip_rot);
            let n_dir = neighbor_direction(loc, *neighbor);
            let edge_to_match = n_edges[match n_dir {
                Direction::Up => 2,
                Direction::Right => 3,
                Direction::Down => 0,
                Direction::Left => 1,
            }];
            for (i, &edge) in tile.raw_edge_ids.iter().enumerate() {
                if edge == edge_to_match {
                    let flip = true;
                    let rotation = flip_rot_i_to_dir(i, n_dir, true);
                    return (flip, rotation);
                } else if edge == reverse_edge(edge_to_match) {
                    let flip = false;
                    let rotation = flip_rot_i_to_dir(i, n_dir, false);
                    return (flip, rotation);
                }
            }
        }
    }
    unreachable!()
}

fn flip_rot_i_to_dir(mut i: usize, dir: Direction, flip: bool) -> Rotation {
    if flip {
        i = match i {
            1 => 3,
            3 => 1,
            _ => i,
        };
    }
    let rot_i_to_up = 4 - i;
    let rot_up_to_dir = match dir {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    };
    match (rot_i_to_up + rot_up_to_dir)%4 {
        0 => Rotation::_0,
        1 => Rotation::_90,
        2 => Rotation::_180,
        3 => Rotation::_270,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Direction { Up, Down, Left, Right }
fn neighbor_direction(base: Point, neighbor: Point) -> Direction {
    if base.0 < neighbor.0 {
        Direction::Right
    } else if base.0 > neighbor.0 {
        Direction::Left
    } else if base.1 < neighbor.1 {
        Direction::Up
    } else if base.1 > neighbor.1 {
        Direction::Down
    } else {
        unreachable!()
    }
}

fn fr_edges(edges: &[i32], (flip, rot): (bool, Rotation)) -> [i32; 4] {
    let idx_list = match flip {
        false => match rot {
            Rotation::_0 => [0, 1, 2, 3],
            Rotation::_90 => [3, 0, 1, 2],
            Rotation::_180 => [2, 3, 0, 1],
            Rotation::_270 => [1, 2, 3, 0],
        }
        true => match rot {
            Rotation::_0 => [0, 3, 2, 1],
            Rotation::_90 => [1, 0, 3, 2],
            Rotation::_180 => [2, 1, 0, 3],
            Rotation::_270 => [3, 2, 1, 0],
        }
    };

    if !flip {
        [edges[idx_list[0]], edges[idx_list[1]], edges[idx_list[2]], edges[idx_list[3]]]
    } else {
        [
            reverse_edge(edges[idx_list[0]]),
            reverse_edge(edges[idx_list[1]]),
            reverse_edge(edges[idx_list[2]]),
            reverse_edge(edges[idx_list[3]]),
        ]
    }
}

fn reverse_edge(mut edge: i32) -> i32 {
    let mut new_edge = 0;
    for _ in 0..10 {
        new_edge = new_edge << 1 | (edge & 1);
        edge >>= 1;
    }
    new_edge
}

type Grid = Vec<Vec<bool>>;
type Point = (i32, i32);

#[derive(Debug,Clone)]
struct Tile {
    id: i64,
    edge_ids: Vec<i32>,
    raw_edge_ids: Vec<i32>,
    inner_grid: Grid,
}

fn parse_input(lines: &[String]) -> Vec<Tile> {
    enum ParseMode {
        NewTile,
        TileLine,
    }

    let mut parse_mode = ParseMode::NewTile;
    let mut tiles = Vec::new();
    let mut current_id: i64 = 0;
    let mut current_grid: Grid = Vec::new();
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
                    // raw follows the edges in a clockwise direction
                    let (mut top_raw, mut top_alt) = (0, 0);
                    let (mut right_raw, mut right_alt) = (0, 0);
                    let (mut bottom_raw, mut bottom_alt) = (0, 0);
                    let (mut left_raw, mut left_alt) = (0, 0);
                    for i in 0..10 {
                        top_raw    = top_raw    << 1 | current_grid[0][  i] as i32;
                        top_alt    = top_alt    << 1 | current_grid[0][9-i] as i32;
                        right_raw  = right_raw  << 1 | current_grid[  i][9] as i32;
                        right_alt  = right_alt  << 1 | current_grid[9-i][9] as i32;
                        bottom_raw = bottom_raw << 1 | current_grid[9][9-i] as i32;
                        bottom_alt = bottom_alt << 1 | current_grid[9][  i] as i32;
                        left_raw   = left_raw   << 1 | current_grid[9-i][0] as i32;
                        left_alt   = left_alt   << 1 | current_grid[  i][0] as i32;
                    }
                    let top = std::cmp::min(top_raw, top_alt);
                    let right = std::cmp::min(right_raw, right_alt);
                    let bottom = std::cmp::min(bottom_raw, bottom_alt);
                    let left = std::cmp::min(left_raw, left_alt);

                    let inner_grid: Grid = current_grid[1..9].iter().map(|row| row[1..9].to_vec()).collect();
                    tiles.push(Tile{
                        id: current_id,
                        edge_ids: vec![top, right, bottom, left],
                        raw_edge_ids: vec![top_raw, right_raw, bottom_raw, left_raw],
                        inner_grid: inner_grid,
                    });
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
