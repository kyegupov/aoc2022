use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let mut max = 0i64;
    let mut acc = 0i64;
    for (i, line) in read_to_string("input01.txt")?.lines().enumerate() {
        let x = line.parse::<i64>();
        match x {
            Ok(xx) => {
                acc += xx;
            }
            Err(x) => {
                if acc > max {
                    max = acc
                };
                acc = 0;
            }
        }
    }
    if acc > max {
        max = acc
    };
    println!("{max}");

    Ok(())
}
