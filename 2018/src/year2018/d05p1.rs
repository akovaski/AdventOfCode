use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day05.txt")?;

    let mut v = Vec::new();
    f.read_to_end(&mut v)?;

    let mut out: Vec<char> = Vec::new();

    for b in v {
        let c = b as char;

        let last_c = out.last();

        if let Some(&last_c) = last_c {
            if c.to_ascii_uppercase() == last_c.to_ascii_uppercase() && c != last_c {
                out.pop();
                continue;
            }
        }

        out.push(c);
    }

    println!("{}", out.len());

    Ok(())
}
