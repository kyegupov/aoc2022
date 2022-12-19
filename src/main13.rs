use std::{env, error::Error, fmt::Debug, fs::read_to_string, ops::Index};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit0, digit1},
    combinator::map,
    error,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

type Value = Vec<Item>;

#[derive(Eq, PartialEq, Clone)]
enum Item {
    Number(u64),
    List(Vec<Item>),
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::List(arg0) => f.write_fmt(format_args!("{:?}", arg0)),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Number(x), Item::Number(y)) => x.cmp(y),
            (Item::Number(x), Item::List(_)) => Item::List(vec![self.clone()]).cmp(other),
            (Item::List(_), Item::Number(y)) => self.cmp(&Item::List(vec![other.clone()])),
            (Item::List(x), Item::List(y)) => {
                for (a, b) in x.iter().zip(y.iter()) {
                    let c = a.cmp(b);
                    if !c.is_eq() {
                        return c;
                    }
                }
                return x.len().cmp(&y.len());
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn number(x: &str) -> IResult<&str, u64> {
    map(digit1::<_, error::Error<_>>, |x: &str| {
        x.parse::<u64>().unwrap()
    })(x)
}

fn list(x: &str) -> IResult<&str, Value> {
    delimited(
        tag("["),
        separated_list0(
            tag(","),
            alt((
                map(list, |x| Item::List(x)),
                map(number, |x| Item::Number(x)),
            )),
        ),
        tag("]"),
    )(x)
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut acc = 0;
    let divider1 = list("[[2]]").unwrap().1;
    let divider2 = list("[[6]]").unwrap().1;
    let mut all_packets = vec![];
    for (i, chunk) in read_to_string("input13.txt")?
        .lines()
        .chunks(3)
        .into_iter()
        .enumerate()
    {
        let (first_str, second_str) = chunk.take(2).collect_tuple().unwrap();
        let first = list(first_str).unwrap().1;
        let second = list(second_str).unwrap().1;

        if part2 {
        } else {
            if second > first {
                acc += i + 1;
            }
        }
        all_packets.push(first);
        all_packets.push(second);
    }
    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());

    if part2 {
        all_packets.sort();
        dbg!(
            (all_packets.iter().position(|x| x == &divider2).unwrap() + 1)
                * (all_packets.iter().position(|x| x == &divider1).unwrap() + 1)
        );
    } else {
        dbg!(acc);
    }

    Ok(())
}
