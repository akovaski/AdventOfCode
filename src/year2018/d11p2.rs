use std::fs::File;
use std::io;
use std::io::prelude::*;

use super::d11p1;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day11.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let serial_num: i32 = buf.parse().unwrap();
    let partial_sums = d11p1::calc_partial_sums(serial_num);

    let mut max_pl = -9999;
    let mut max_coord = (0, 0, 0);

    for size in 1..=300 {
        for x in 0..300 - size + 1 {
            for y in 0..300 - size + 1 {
                let square_pl = d11p1::square_power_level(&partial_sums, x, y, size);

                if square_pl > max_pl {
                    max_pl = square_pl;
                    max_coord = (x, y, size);
                }
            }
        }
    }

    println!("{:?}", max_coord);

    Ok(())
}
