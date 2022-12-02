use std::{collections::BTreeSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let mut all = BTreeSet::new();
    let mut acc = 0i64;
    for (i, line) in read_to_string("input01.txt")?.lines().enumerate() {
        let x = line.parse::<i64>();
        match x {
            Ok(xx) => {
                acc += xx;
            }
            Err(x) => {
                all.insert(acc);
                acc = 0;
            }
        }
    }
    all.insert(acc);
    println!("{}", all.iter().rev().take(3).sum::<i64>());

    Ok(())
}
