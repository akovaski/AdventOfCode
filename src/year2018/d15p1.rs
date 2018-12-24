use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Eq)]
pub enum Tile {
    Empty, // .
    Wall,  // #
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Unit {
    Elf,
    Goblin,
}

impl Unit {
    pub fn enemy(&self) -> Unit {
        match *self {
            Unit::Elf => Unit::Goblin,
            Unit::Goblin => Unit::Elf,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CharInfo {
    pub unit: Unit,
    pub health: i32,
    pub moved: bool,
}

// y coordinate takes precedent over x when ordering,
// this allows us to advance carts in Point's order
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            y_order => y_order,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    fn north(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    // adjacent points, in order
    pub fn adjacent_points(&self) -> [Point; 4] {
        [self.north(), self.west(), self.east(), self.south()]
    }
}

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day15.txt")?;
    let reader = BufReader::new(f);

    let mut map: BTreeMap<Point, Tile> = BTreeMap::new();

    let mut chars: BTreeMap<Point, CharInfo> = BTreeMap::new();

    let init_health = 200;

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            let p = Point { x, y };
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'E' => {
                    chars.insert(
                        p,
                        CharInfo {
                            unit: Unit::Elf,
                            health: init_health,
                            moved: false,
                        },
                    );
                    Tile::Empty
                }
                'G' => {
                    chars.insert(
                        p,
                        CharInfo {
                            unit: Unit::Goblin,
                            health: init_health,
                            moved: false,
                        },
                    );
                    Tile::Empty
                }
                _ => panic!("unrecognized map char"),
            };

            map.insert(p, tile);
        }
    }

    for round in 0.. {
        let complete_round = simulate_one_round(&map, &mut chars);

        let goblins_remaining = chars.values().any(|&c| c.unit == Unit::Goblin);
        let elfs_remaining = chars.values().any(|&c| c.unit == Unit::Elf);
        if !goblins_remaining || !elfs_remaining {
            let last_full_round = match complete_round {
                true => round + 1,
                false => round,
            };
            let remaining_hp: i32 = chars.values().map(|c| c.health).sum();
            println!("Outcome: {}", last_full_round * remaining_hp);
            break;
        }
    }

    Ok(())
}

fn simulate_one_round(map: &BTreeMap<Point, Tile>, chars: &mut BTreeMap<Point, CharInfo>) -> bool {
    for point in chars.clone().keys() {
        let unit = chars.get(point);
        if unit.is_none() || unit.unwrap().moved {
            continue;
        }
        let unit = unit.unwrap().clone();

        let enemies_remaining = chars.values().any(|&c| c.unit == unit.unit.enemy());
        if !enemies_remaining {
            return false;
        }

        // find the tile the unit wants to move to
        let found = find_target_position_tile(&map, &chars, point, &unit);

        // new_point is where the unit is after their movement turn
        let new_point = if let Some(found) = found {
            let step = find_shortest_step(&map, &chars, &found, point);
            if step != *point {
                chars.remove(point);
                chars.insert(step, unit);
            }
            step
        } else {
            *point
        };

        let adj_enemies: Vec<Point> = new_point
            .adjacent_points()
            .iter()
            .filter_map(|adj| match chars.get(adj) {
                Some(adj_unit) => match adj_unit.unit == unit.unit {
                    true => None,
                    false => Some(*adj),
                },
                None => None,
            })
            .collect();

        if adj_enemies.len() > 0 {
            let min_hp = adj_enemies.iter().map(|p| chars[p].health).min().unwrap();

            // adj_enemies is already in reading order, so all we have to do is filter
            // the enemies with the least HP and take the first one.
            let point_to_attack = adj_enemies
                .iter()
                .filter(|&p| chars[p].health == min_hp)
                .next()
                .unwrap();

            let enemy = chars.get_mut(point_to_attack).unwrap();
            enemy.health -= 3;

            if enemy.health <= 0 {
                chars.remove(point_to_attack);
            }
        }
    }
    true
}

fn find_target_position_tile(
    map: &BTreeMap<Point, Tile>,
    chars: &BTreeMap<Point, CharInfo>,
    point: &Point,
    unit: &CharInfo,
) -> Option<Point> {
    let enemy_unit = unit.unit.enemy();
    let mut distance: HashMap<Point, i32> = HashMap::new();

    distance.insert(*point, 0);

    let mut enemy_distance = None;

    let mut last_set = vec![*point];
    // first find enemies within search radius
    // all target locations will be one less step away
    for search_radius in 1.. {
        let mut new_set: Vec<Point> = Vec::new();

        for &point in &last_set {
            for adj in &point.adjacent_points() {
                if !distance.contains_key(adj) && map[adj] == Tile::Empty {
                    if let Some(unit) = chars.get(adj) {
                        if unit.unit != enemy_unit {
                            continue;
                        }
                    }
                    new_set.push(*adj);
                    distance.insert(*adj, search_radius);
                }
            }
        }

        for point in &new_set {
            if let Some(unit) = chars.get(point) {
                if unit.unit == enemy_unit {
                    enemy_distance = Some(search_radius);
                    break;
                }
            }
        }

        if new_set.len() == 0 {
            break;
        }

        last_set = new_set;

        if enemy_distance.is_some() {
            break;
        }
    }

    if enemy_distance.is_none() {
        // no enemies within range
        return None;
    }

    let enemy_distance = enemy_distance.unwrap();

    // last_set now has the points of all closest enemies and other locations
    // let's filter that down to just enemies
    let enemies = last_set.iter().filter(|&p| match chars.get(p) {
        Some(unit) => {
            if unit.unit == enemy_unit {
                true
            } else {
                false
            }
        }
        None => false,
    });

    let nearset_adj = enemies
        .map(|p| {
            let adj_points = p.adjacent_points();
            adj_points
                .iter()
                .filter(|&adj| match distance.get(adj) {
                    Some(&dist) => dist < enemy_distance,
                    None => false,
                })
                .map(|&p| p)
                .collect::<Vec<Point>>()
        })
        .flatten();

    // create a min-heap (reverse max-heap) to get adjacent points in reading order
    let ordered_adj: BinaryHeap<_> = nearset_adj
        .map(|p| Reverse(p))
        .collect::<BinaryHeap<Reverse<Point>>>();
    Some(ordered_adj.peek().unwrap().0)
}

fn find_shortest_step(
    map: &BTreeMap<Point, Tile>,
    chars: &BTreeMap<Point, CharInfo>,
    start: &Point,
    destination: &Point,
) -> Point {
    if start == destination {
        return *start;
    }

    let mut distance: HashMap<Point, i32> = HashMap::new();

    distance.insert(*start, 0);

    let mut dest_distance = None;

    let mut last_set = vec![*start];
    // first find enemies within search radius
    // all target locations will be one less step away
    for search_radius in 1.. {
        let mut new_set: Vec<Point> = Vec::new();

        for &point in &last_set {
            for adj in &point.adjacent_points() {
                if !distance.contains_key(adj) && map[adj] == Tile::Empty {
                    if chars.get(adj).is_some() {
                        if adj == destination {
                            dest_distance = Some(search_radius);
                        } else {
                            continue;
                        }
                    }
                    new_set.push(*adj);
                    distance.insert(*adj, search_radius);
                }
            }
        }

        if new_set.len() == 0 {
            break;
        }

        last_set = new_set;

        if dest_distance.is_some() {
            break;
        }
    }

    if dest_distance.is_none() {
        // no enemies within range
        panic!("Right now find_shortest_step should only be called when there is a confirmed path");
    }

    let dest_distance = dest_distance.unwrap();

    let ordered_adj = destination
        .adjacent_points()
        .iter()
        .filter(|&p| match distance.get(p) {
            Some(&dist) => dist < dest_distance,
            None => false,
        })
        .map(|&p| Reverse(p))
        .collect::<BinaryHeap<Reverse<Point>>>();

    ordered_adj.peek().unwrap().0
}

#[allow(dead_code)]
fn print_map(map: &BTreeMap<Point, Tile>, chars: &BTreeMap<Point, CharInfo>) {
    for (point, tile) in map.iter() {
        if point.y != 0 && point.x == 0 {
            println!();
        }

        let cc = if let Some(character) = chars.get(&point) {
            match character.unit {
                Unit::Elf => "E",
                Unit::Goblin => "G",
            }
        } else {
            match tile {
                Tile::Empty => ".",
                Tile::Wall => "#",
            }
        };
        print!("{}", cc);
    }
    println!();
}
