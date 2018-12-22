use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub enum Track {
    Vertical,     //  |
    Horizontal,   //  --
    DiagonalDown, // '-\'
    DiagonalUp,   // '-/'
    Intersect,    //  +
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Cart {
    Up(Turn),
    Down(Turn),
    Left(Turn),
    Right(Turn),
    Crash,
}

// y coordinate takes precedent over x when ordering,
// this allows us to advance carts in Point's order
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
        simulate_one_tick(&map, &mut carts);

        if carts.values().filter(|&&c| c == Cart::Crash).count() > 0 {
            break;
        }
    }

    let crash_point: Point = carts
        .iter()
        .filter_map(|(&point, &cart)| match cart {
            Cart::Crash => Some(point),
            _ => None,
        })
        .next()
        .unwrap();
    println!("{},{}", crash_point.x, crash_point.y);

    Ok(())
}

pub fn simulate_one_tick(map: &BTreeMap<Point, Track>, carts: &mut BTreeMap<Point, Cart>) {
    let clone = carts.clone();
    let prev_carts: Vec<&Point> = clone.keys().collect();
    for point in prev_carts {
        let (mut new_cart, new_point) = match carts[point] {
            Cart::Crash => (Cart::Crash, *point),
            Cart::Up(turn) => match map[point] {
                Track::Vertical => move_cart_straight(&Cart::Up(turn), point),
                Track::DiagonalUp => move_cart_straight(&Cart::Right(turn), point),
                Track::DiagonalDown => move_cart_straight(&Cart::Left(turn), point),
                Track::Intersect => {
                    let new_turn = get_next_turn(turn);
                    let new_cart = match turn {
                        Turn::Left => Cart::Left(new_turn),
                        Turn::Straight => Cart::Up(new_turn),
                        Turn::Right => Cart::Right(new_turn),
                    };
                    move_cart_straight(&new_cart, point)
                }
                _ => panic!("not a valid track for Up Cart"),
            },
            Cart::Down(turn) => match map[point] {
                Track::Vertical => move_cart_straight(&Cart::Down(turn), point),
                Track::DiagonalUp => move_cart_straight(&Cart::Left(turn), point),
                Track::DiagonalDown => move_cart_straight(&Cart::Right(turn), point),
                Track::Intersect => {
                    let new_turn = get_next_turn(turn);
                    let new_cart = match turn {
                        Turn::Left => Cart::Right(new_turn),
                        Turn::Straight => Cart::Down(new_turn),
                        Turn::Right => Cart::Left(new_turn),
                    };
                    move_cart_straight(&new_cart, point)
                }
                _ => panic!("not a valid track for Down Cart"),
            },
            Cart::Left(turn) => match map[point] {
                Track::Horizontal => move_cart_straight(&Cart::Left(turn), point),
                Track::DiagonalUp => move_cart_straight(&Cart::Down(turn), point),
                Track::DiagonalDown => move_cart_straight(&Cart::Up(turn), point),
                Track::Intersect => {
                    let new_turn = get_next_turn(turn);
                    let new_cart = match turn {
                        Turn::Left => Cart::Down(new_turn),
                        Turn::Straight => Cart::Left(new_turn),
                        Turn::Right => Cart::Up(new_turn),
                    };
                    move_cart_straight(&new_cart, point)
                }
                _ => panic!("not a valid track for Left Cart"),
            },
            Cart::Right(turn) => match map[point] {
                Track::Horizontal => move_cart_straight(&Cart::Right(turn), point),
                Track::DiagonalUp => move_cart_straight(&Cart::Up(turn), point),
                Track::DiagonalDown => move_cart_straight(&Cart::Down(turn), point),
                Track::Intersect => {
                    let new_turn = get_next_turn(turn);
                    let new_cart = match turn {
                        Turn::Left => Cart::Up(new_turn),
                        Turn::Straight => Cart::Right(new_turn),
                        Turn::Right => Cart::Down(new_turn),
                    };
                    move_cart_straight(&new_cart, point)
                }
                _ => panic!("not a valid track for Right Cart"),
            },
        };

        if carts.contains_key(&new_point) {
            new_cart = Cart::Crash;
        }

        carts.remove(point).unwrap();
        carts.insert(new_point, new_cart);
    }
}

fn get_next_turn(turn: Turn) -> Turn {
    match turn {
        Turn::Left => Turn::Straight,
        Turn::Straight => Turn::Right,
        Turn::Right => Turn::Left,
    }
}

fn move_cart_straight(cart: &Cart, point: &Point) -> (Cart, Point) {
    (
        *cart,
        match cart {
            Cart::Up(_) => Point {
                x: point.x,
                y: point.y - 1,
            },
            Cart::Down(_) => Point {
                x: point.x,
                y: point.y + 1,
            },
            Cart::Left(_) => Point {
                x: point.x - 1,
                y: point.y,
            },
            Cart::Right(_) => Point {
                x: point.x + 1,
                y: point.y,
            },
            Cart::Crash => *point,
        },
    )
}
