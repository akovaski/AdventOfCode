use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    assert!(lines.len() == 2);

    let busses: Vec<(i64, i64)> = lines[1].split(',').enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(i, b)| (i as i64, b.parse::<i64>().unwrap()))
        .collect();

    let (mut prev_offset, mut prev_period) = busses[0];

    for &(bus_offset, bus_period) in &busses[1..] {
        for i in 0..bus_period {
            let timing = prev_offset + prev_period*i;
            if (timing + bus_offset) % bus_period == 0 {
                prev_period = least_common_denominator(prev_period, bus_period);
                prev_offset = timing % prev_period;
                break;
            } else {
                assert!(i != bus_period - 1);
            }
        }
    }

    println!("earliest time: {}", prev_offset);

    Ok(())
}

fn least_common_denominator(a: i64, b: i64) -> i64 {
    let mut ma = std::cmp::min(a, b);
    let mut mb = std::cmp::max(a, b);

    let mut gcd = 1;

    for i in 2..=ma {
        while ma % i == 0 && mb % i == 0 {
            gcd *= i;
            ma /= i;
            mb /= i;
        }
    }
    ma * mb * gcd
}
