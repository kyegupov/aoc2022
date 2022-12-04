use std::{error::Error, fs::read_to_string};

use nom::{
    bytes::complete::tag, character::complete::digit0, combinator::map, error, sequence::tuple,
    IResult,
};

type Parser<'a, T> = fn(&'a str) -> IResult<&'a str, T>;

fn main() -> Result<(), Box<dyn Error>> {
    let number = |x| {
        map(digit0::<_, error::Error<_>>, |x: &str| {
            x.parse::<u64>().unwrap()
        })(x)
    };
    let pair = |x| map(tuple((&number, tag("-"), &number)), |(x, _, y)| (x..=y))(x);
    let pairs = |x| map(tuple((pair, tag(","), pair)), |(x, _, y)| (x, y))(x);

    let mut acc = 0i64;

    for (i, line) in read_to_string("input04.txt")?.lines().enumerate() {
        let p = pairs(line).unwrap().1;
        if (p.1.start() <= p.0.end() && p.1.end() >= p.0.start())
            || (p.1.start() <= p.0.end() && p.1.end() >= p.0.start())
        {
            acc += 1;
        }
    }
    println!("{acc}");

    Ok(())
}
