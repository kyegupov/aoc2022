use std::{env, error::Error, fs::read_to_string};

use nom::{
    bytes::complete::tag, character::complete::digit0, combinator::map, error, sequence::tuple,
    IResult,
};

use itertools::Itertools;

type Parser<'a, T> = fn(&'a str) -> IResult<&'a str, T>;

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let len = if part2 { 14 } else { 4 };

    for (i, line) in read_to_string("input06.txt")?.lines().enumerate() {
        for j in 0..line.len() - len {
            if line[j..j + len].chars().unique().count() == len {
                dbg!(j + len);
                break;
            }
        }
    }

    Ok(())
}
