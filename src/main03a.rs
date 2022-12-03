use std::{collections::BTreeSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let mut acc = 0i64;
    for (i, line) in read_to_string("input03.txt")?.lines().enumerate() {
        let first: BTreeSet<char> = line[0..line.len() / 2].chars().collect();
        let second: BTreeSet<char> = line[line.len() / 2..].chars().collect();
        for c in first.intersection(&second) {
            acc += match c {
                'a'..='z' => (*c as i64 - 'a' as i64 + 1),
                'A'..='Z' => (*c as i64 - 'A' as i64 + 27),
                _ => panic!(),
            }
        }
    }
    println!("{acc}");

    Ok(())
}
