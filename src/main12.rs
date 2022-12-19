use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env,
    error::Error,
    fs::read_to_string,
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
}

const directions: [Coord2D; 4] = [
    Coord2D { x: -1, y: 0 },
    Coord2D { x: 0, y: -1 },
    Coord2D { x: 1, y: 0 },
    Coord2D { x: 0, y: 1 },
];

#[derive(Debug)]
struct Rules {
    map: BTreeMap<Coord2D, char>,
    part2: bool,
}

#[derive(Default, Debug)]
struct State {
    front: VecDeque<Coord2D>,
    distance: BTreeMap<Coord2D, usize>,
}

fn level(c: char, part2: bool) -> u32 {
    if part2 {
        match c {
            'E' => 0,
            'a'..='z' => ('z' as u32 - c as u32) + 1,
            'S' => ('z' as u32 - 'a' as u32) + 1,
            _ => panic!(),
        }
    } else {
        match c {
            'S' => 0,
            'a'..='z' => (c as u32 - 'a' as u32) + 1,
            'E' => ('z' as u32 - 'a' as u32) + 1,
            _ => panic!(),
        }
    }
}

impl State {
    fn do_tick(&mut self, rules: &Rules) -> Option<usize> {
        let coords = self.front.pop_front().unwrap();
        let cc = rules.map[&coords];
        let distance = self.distance[&coords];
        for dir in directions {
            let coords2 = coords.add(dir);
            if rules.map.contains_key(&coords2) {
                if !self.distance.contains_key(&coords2) {
                    let cc2 = rules.map[&coords2];
                    if level(cc2, rules.part2) <= level(cc, rules.part2) + 1 {
                        if rules.part2 {
                            if cc2 == 'a' || cc2 == 'S' {
                                return Some(distance + 1);
                            }
                        } else {
                            if cc2 == 'E' {
                                return Some(distance + 1);
                            }
                        }
                        self.distance.insert(coords2, distance + 1);
                        self.front.push_back(coords2);
                    }
                }
            }
        }
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut rules: Rules = Rules {
        map: Default::default(),
        part2,
    };
    let mut state: State = Default::default();

    for (y, line) in read_to_string("input12.txt")?.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord2D {
                x: x as isize,
                y: y as isize,
            };
            rules.map.insert(coord, c);
            if part2 {
                if c == 'E' {
                    state.front.push_back(coord);
                    state.distance.insert(coord, 0);
                }
            } else {
                if c == 'S' {
                    state.front.push_back(coord);
                    state.distance.insert(coord, 0);
                }
            }
        }
    }
    loop {
        if let Some(res) = state.do_tick(&rules) {
            dbg!(&res);
            break;
        }
    }

    Ok(())
}
