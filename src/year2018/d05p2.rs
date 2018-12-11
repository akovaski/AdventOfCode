use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn main() -> io::Result<()> {
    let mut f = File::open("./input/day05.txt")?;

    let mut v = Vec::new();
    f.read_to_end(&mut v)?;

    let char_vec: Vec<char> = v.iter().map(|&b| b as char).collect();
    let first_pass = reduce_polymer(char_vec);

    let min_reduct = (('a' as u8)..('z' as u8))
        .map(|b| {
            let c = b as char;

            let filtered: Vec<char> = first_pass
                .iter()
                .filter(|fc| fc.to_ascii_lowercase() != c)
                .map(|&fc| fc)
                .collect();
            reduce_polymer(filtered).len()
        })
        .min()
        .unwrap();

    println!("{}", min_reduct);
    Ok(())
}

fn reduce_polymer(in_vec: Vec<char>) -> Vec<char> {
    let mut out_vec: Vec<char> = Vec::new();

    for c in in_vec {
        let last_c = out_vec.last();

        if let Some(&last_c) = last_c {
            if c.to_ascii_uppercase() == last_c.to_ascii_uppercase() && c != last_c {
                out_vec.pop();
                continue;
            }
        }

        out_vec.push(c);
    }

    out_vec
}
