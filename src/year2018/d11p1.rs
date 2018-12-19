use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day11.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let serial_num: i32 = buf.parse().unwrap();

    let mut max_pl = -9999;
    let mut max_coord = (0, 0);

    for x in 0..300 - 2 {
        for y in 0..300 - 2 {
            let mut square_pl = 0;
            for dx in 0..3 {
                for dy in 0..3 {
                    square_pl += power_level(x + dx, y + dy, serial_num);
                }
            }

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
