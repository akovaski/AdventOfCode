use std::fs::File;
use std::io;
use std::io::prelude::*;

use super::d11p1;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day11.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let serial_num: i32 = buf.parse().unwrap();

    let mut max_pl = -9999;
    let mut max_coord = (0, 0, 0);

    let mut cache = [[0; 300]; 300];

    for size in 1..=300 {
        for x in 0..300 - size + 1 {
            for y in 0..300 - size + 1 {
                let square_pl: &mut i32 = &mut cache[x as usize][y as usize];
                for dx in 0..size - 1 {
                    *square_pl += d11p1::power_level(x + dx, y + size - 1, serial_num);
                }
                for dy in 0..size - 1 {
                    *square_pl += d11p1::power_level(x + size - 1, y + dy, serial_num);
                }
                *square_pl += d11p1::power_level(x + size - 1, y + size - 1, serial_num);

                if *square_pl > max_pl {
                    max_pl = *square_pl;
                    max_coord = (x, y, size);
                }
            }
        }
    }

    println!("{:?}", max_coord);

    Ok(())
}
