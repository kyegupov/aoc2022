use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs::read_to_string,
    iter,
};

use nom::{
    bytes::complete::tag, character::complete::digit0, combinator::map, error, sequence::tuple,
    IResult,
};

use itertools::Itertools;

type Parser<'a, T> = fn(&'a str) -> IResult<&'a str, T>;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Entry {
    Dir(BTreeMap<String, Entry>),
    File(usize),
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let rope_len = if part2 { 10 } else { 2 };

    let mut rope = iter::repeat((0i32, 0i32)).take(rope_len).collect_vec();
    let mut tail_visited = BTreeSet::new();

    for (i, line) in read_to_string("input09.txt")?.lines().enumerate() {
        let mut words = line.split_whitespace();
        let (dx, dy) = match words.next().unwrap() {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => panic!(),
        };
        for _ in 0usize..words.next().unwrap().parse().unwrap() {
            let head = rope.get_mut(0).unwrap();
            head.0 += dx;
            head.1 += dy;
            let mut prev = rope[0];
            for part in rope.iter_mut().skip(1) {
                let (ddx, ddy) = (prev.0 - part.0, prev.1 - part.1);
                if ddx.abs() > 1 || ddy.abs() > 1 {
                    part.0 += ddx.signum();
                    part.1 += ddy.signum();
                };
                prev = *part;
            }
            tail_visited.insert(*rope.last().unwrap());
        }
        // dbg!(head, tail);
    }

    dbg!(tail_visited.len());

    Ok(())
}
