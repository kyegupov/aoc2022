use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let mut acc = 0i64;
    for (i, line) in read_to_string("input02.txt")?.lines().enumerate() {
        let h1 = &line[0..1];
        let h2 = &line[2..3];
        let val1 = match h1 {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => {
                panic!()
            }
        };
        let val2 = match h2 {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => {
                panic!()
            }
        };
        acc += val2;
        let win = match (val2 - val1).rem_euclid(3) {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => {
                panic!()
            }
        };
        acc += win;
    }
    println!("{acc}");

    Ok(())
}
