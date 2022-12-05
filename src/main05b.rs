use std::{error::Error, fs::read_to_string};

use nom::{
    bytes::complete::tag, character::complete::digit0, combinator::map, error, sequence::tuple,
    IResult,
};

use itertools::Itertools;

type Parser<'a, T> = fn(&'a str) -> IResult<&'a str, T>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    for (i, line) in read_to_string("input05.txt")?.lines().enumerate() {
        if line.contains("[") {
            let chars = line.chars().collect_vec();
            for i in 0..((chars.len() + 3) / 4) {
                while stacks.len() < i + 1 {
                    stacks.push(vec![]);
                }
                let c = chars[i * 4 + 1];
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
        }
        if line == "" {
            for s in stacks.iter_mut() {
                s.reverse();
            }
        }
        if line.contains("move") {
            let words = line.split(" ").collect_vec();
            let qty: usize = words[1].parse()?;
            let from: usize = words[3].parse()?;
            let to: usize = words[5].parse()?;
            let mut mover = vec![];
            for i in 0..qty {
                let c = stacks[from - 1].pop().unwrap();
                mover.push(c);
            }
            mover.reverse();
            stacks[to - 1].extend(mover);
        }
    }

    let s = stacks.iter().map(|s| s.last().unwrap()).join("");
    println!("{s}");

    Ok(())
}
