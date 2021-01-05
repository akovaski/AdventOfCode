use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    assert!(lines.len() == 2);
    let keys: Vec<i64> = lines.into_iter().map(|l| l.parse().unwrap()).collect();

    let subject = 7;
    
    let key_mod: i64 = 20201227;
    let mut key_gen: i64 = 1;
    let mut key_0_loop = None;
    for key_loop in 1..key_mod {
        key_gen = (key_gen * subject) % key_mod;
        if key_gen == keys[0] {
            key_0_loop = Some(key_loop);
            break;
        }
    }
    let key_0_loop = key_0_loop.unwrap();
    println!("Key {} Loop {}", key_gen, key_0_loop);

    let mut encryption_key: i64 = 1;
    for _ in 0..key_0_loop {
        encryption_key = (encryption_key * keys[1]) % key_mod;
    }
    println!("Encryption Key: {}", encryption_key);

    Ok(())
}
