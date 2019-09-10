extern crate sled;

use sled::Db;
use std::fs::*;
use std::io::*;

fn parse_vecu8(s: &str) -> Vec<u8> {
    let l = s.len();
    s[1..l - 1].split(", ").map(|xs| xs.parse::<u8>().unwrap()).collect()
}

fn main() {
    let db = Db::open("sled_debug.db").unwrap();
    let f = File::open("data").unwrap();
    let file = BufReader::new(&f);
    println!("starting to dump data into the db");
    let mut counter = 0u32;
    for line in file.lines() {
        let l = line.unwrap();
        let x: Vec<&str> = l.split('#').collect();
        db.insert(parse_vecu8(x[0]), parse_vecu8(x[1])).unwrap();
        counter += 1;
    }
    println!("{} record(s) inserted", counter);
    println!("trying to scan the db from the beginning, {}", db.len());
}
