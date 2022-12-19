use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env,
    error::Error,
    fs::read_to_string,
};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while},
    character::{complete::digit1, is_digit},
    combinator::{map, map_res, recognize},
    error,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    AsChar, IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord2D {
    x: i64,
    y: i64,
}

impl Coord2D {
    fn add(self, o: Self) -> Self {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
        }
    }
    fn sub(self, o: Self) -> Self {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
        }
    }

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

const directions4: [Coord2D; 4] = [
    Coord2D { x: -1, y: 0 },
    Coord2D { x: 0, y: -1 },
    Coord2D { x: 1, y: 0 },
    Coord2D { x: 0, y: 1 },
];

#[derive(Debug)]
struct Rules {
    sensors_beacons: Vec<(Coord2D, Coord2D)>,
}

#[derive(Default, Debug)]
struct State {}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(
        take_while(|x: char| x == '-' || x.is_dec_digit()),
        str::parse,
    )(input)
}

fn coord(x: &str) -> IResult<&str, Coord2D> {
    map(
        separated_pair(preceded(tag("x="), parse_i64), tag(", y="), parse_i64),
        |(x, y)| Coord2D {
            x: x as i64,
            y: y as i64,
        },
    )(x)
}

fn parse_line(x: &str) -> IResult<&str, (Coord2D, Coord2D)> {
    separated_pair(
        preceded(tag("Sensor at "), coord),
        tag(": closest beacon is at "),
        coord,
    )(x)
}

impl State {
    fn do_tick(&mut self, rules: &Rules) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut rules: Rules = Rules {
        sensors_beacons: vec![],
    };

    for (y, line) in read_to_string("input15.txt")?.lines().enumerate() {
        let pair = parse_line(line).unwrap().1;
        rules.sensors_beacons.push(pair);
    }
    if part2 {
        'out: for yy in 0..=4000000 {
            let mut excluded_ranges = BTreeSet::new();
            for (s, b) in &rules.sensors_beacons {
                let d = s.sub(*b).manhattan() as i64;
                let dx = d - (yy - s.y).abs() as i64;
                if dx > 0 {
                    excluded_ranges.insert((s.x - dx, s.x + dx));
                }
            }
            let mut x_candidate = 0;
            for (s, e) in &excluded_ranges {
                if *s <= x_candidate {
                    if *e >= x_candidate {
                        x_candidate = *e + 1;
                        if x_candidate >= 4000000 {
                            break;
                        }
                    }
                } else {
                    dbg!(x_candidate, yy);
                    dbg!(4000_000 * x_candidate + yy);
                    break 'out;
                }
            }
        }
    } else {
        let yy = 2000000;
        let mut acc = BTreeSet::new();
        for (s, b) in &rules.sensors_beacons {
            let d = s.sub(*b).manhattan() as i64;
            let dx = d - (yy - s.y).abs() as i64;
            if dx > 0 {
                acc.extend(s.x - dx..=s.x + dx);
            }
        }
        for (s, b) in rules.sensors_beacons {
            if b.y == yy {
                acc.remove(&b.x);
            }
        }
        dbg!(acc.len());
    }

    Ok(())
}
