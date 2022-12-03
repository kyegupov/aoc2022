use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let mut acc = 0i64;
    for (i, line) in read_to_string("input02.txt")?.lines().enumerate() {
        let h1 = &line[0..1];
        let h2 = &line[2..3];
        let val1: i64 = match h1 {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => {
                panic!()
            }
        };
        let (win, val2) = match h2 {
            "X" => (0, (val1 - 1).rem_euclid(3)),
            "Y" => (3, val1),
            "Z" => (6, (val1 + 1).rem_euclid(3)),
            _ => {
                panic!()
            }
        };
        acc += val2 + 1;
        acc += win;
    }
    println!("{acc}");

    Ok(())
}
