use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::read_to_string,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut acc = 0i64;
    for lines in &read_to_string("input03.txt")?.lines().chunks(3) {
        let mut counter: BTreeMap<char, usize> = BTreeMap::new();
        for line in lines {
            for c in line.chars().unique() {
                *counter.entry(c).or_default() += 1;
            }
        }
        for (c, co) in counter {
            if co == 3 {
                acc += match c {
                    'a'..='z' => (c as i64 - 'a' as i64 + 1),
                    'A'..='Z' => (c as i64 - 'A' as i64 + 27),
                    _ => panic!(),
                }
            }
        }
    }
    println!("{acc}");

    Ok(())
}
