use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day11.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let serial_num: i32 = buf.parse().unwrap();
    let partial_sums = calc_partial_sums(serial_num);

    let mut max_pl = -9999;
    let mut max_coord = (0, 0);

    for x in 0..300 - 2 {
        for y in 0..300 - 2 {
            let square_pl = square_power_level(&partial_sums, x, y, 3);

            if square_pl > max_pl {
                max_pl = square_pl;
                max_coord = (x, y);
            }
        }
    }

    println!("{:?}", max_coord);

    Ok(())
}

pub fn power_level(x: i32, y: i32, serial_num: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_num;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;
    power_level
}

pub fn calc_partial_sums(serial_num: i32) -> [[i32; 300]; 300] {
    let mut partial_sums = [[0; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            let mut ps = power_level(x as i32, y as i32, serial_num);

            if x > 0 {
                ps += partial_sums[x - 1][y];
            }
            if y > 0 {
                ps += partial_sums[x][y - 1];
            }
            if x > 0 && y > 0 {
                ps -= partial_sums[x - 1][y - 1];
            }
            partial_sums[x][y] = ps;
        }
    }

    partial_sums
}

pub fn square_power_level(
    partial_sums: &[[i32; 300]; 300],
    x: usize,
    y: usize,
    size: usize,
) -> i32 {
    let mut square_pl = partial_sums[x + size - 1][y + size - 1];
    if x > 0 {
        square_pl -= partial_sums[x - 1][y + size - 1];
    }
    if y > 0 {
        square_pl -= partial_sums[x + size - 1][y - 1];
    }
    if x > 0 && y > 0 {
        square_pl += partial_sums[x - 1][y - 1];
    }

    square_pl
}
