use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env,
    error::Error,
    fs::read_to_string,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, error,
    multi::separated_list1, sequence::separated_pair, IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord2D {
    x: isize,
    y: isize,
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
}

const directions: [Coord2D; 4] = [
    Coord2D { x: -1, y: 0 },
    Coord2D { x: 0, y: -1 },
    Coord2D { x: 1, y: 0 },
    Coord2D { x: 0, y: 1 },
];

#[derive(Debug)]
struct Rules {
    bottom: isize,
    part2: bool,
}

#[derive(Default, Debug)]
struct State {
    map: BTreeMap<Coord2D, char>,
}

fn number(x: &str) -> IResult<&str, u64> {
    map(digit1::<_, error::Error<_>>, |x: &str| {
        x.parse::<u64>().unwrap()
    })(x)
}

fn coord(x: &str) -> IResult<&str, Coord2D> {
    map(separated_pair(number, tag(","), number), |(x, y)| Coord2D {
        x: x as isize,
        y: y as isize,
    })(x)
}

fn list(x: &str) -> IResult<&str, Vec<Coord2D>> {
    separated_list1(tag(" -> "), coord)(x)
}

impl State {
    fn do_tick(&mut self, rules: &Rules) -> bool {
        let mut sand = Coord2D { x: 500, y: 0 };
        if self.map.contains_key(&sand) {
            return true;
        }
        loop {
            let moved = [0, -1, 1]
                .iter()
                .map(|dx| Coord2D {
                    x: sand.x + *dx,
                    y: sand.y + 1,
                })
                .filter(|sand2| !self.map.contains_key(sand2))
                .next();
            if let Some(sand2) = moved {
                sand = sand2;
                if sand2.y > rules.bottom {
                    if rules.part2 {
                        if sand.y == rules.bottom + 1 {
                            self.map.insert(sand, 'o');
                            return false;
                        }
                    } else {
                        return true;
                    }
                }
            } else {
                self.map.insert(sand, 'o');
                return false;
            }
        }
    }
}

fn print(map: &BTreeMap<Coord2D, char>) {
    let yr = map.keys().map(|c| c.y).minmax().into_option().unwrap();
    let xr = map.keys().map(|c| c.x).minmax().into_option().unwrap();
    for y in 0..=yr.1 {
        for x in xr.0..=xr.1 {
            print!("{}", map.get(&Coord2D { x, y }).unwrap_or(&'.'));
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut rules: Rules = Rules { bottom: 0, part2 };
    let mut state: State = Default::default();

    for (y, line) in read_to_string("input14.txt")?.lines().enumerate() {
        let walls = list(line).unwrap().1;
        for (start, end) in walls.iter().zip(walls.iter().skip(1)) {
            let dir = end.sub(*start).signum();
            state.map.insert(*start, '#');
            let mut pos = *start;
            while pos != *end {
                pos = pos.add(dir);
                state.map.insert(pos, '#');
            }
        }
    }
    rules.bottom = state.map.keys().map(|c| c.y).max().unwrap();
    loop {
        if state.do_tick(&rules) {
            print(&state.map);
            dbg!(state.map.values().filter(|x| **x == 'o').count());
            break;
        }
    }

    Ok(())
}
