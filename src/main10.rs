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

#[derive(Default, Debug)]
struct State {
    x: i64,
    acc: i64,
    tick: usize,
    crt: Vec<Vec<bool>>,
    part2: bool,
}

impl State {
    fn do_tick(&mut self) {
        if self.part2 {
            let xx = (self.tick) % 40;
            if xx == 0 {
                self.crt.push(vec![]);
            }
            let sprite = self.x;
            let lit = (xx as isize - sprite as isize).abs() <= 1;
            self.crt.last_mut().unwrap().push(lit);
        }
        self.tick += 1;
        if !self.part2 {
            if self.tick % 40 == 20 {
                self.acc += self.tick as i64 * self.x;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut state = State {
        x: 1,
        tick: 0,
        part2,
        ..Default::default()
    };
    state.do_tick();
    for (i, line) in read_to_string("input10.txt")?.lines().enumerate() {
        let words = line.split_whitespace().collect_vec();
        match words[0] {
            "noop" => {}
            "addx" => {
                state.do_tick();
                state.x += words[1].parse::<i64>().unwrap();
            }
            _ => panic!(),
        }
        state.do_tick();
    }

    if part2 {
        println!(
            "{}",
            state
                .crt
                .iter()
                .map(|l| l.iter().map(|c| if *c { "#" } else { "." }).join(""))
                .join("\n")
        );
    } else {
        dbg!(state.acc);
    }

    Ok(())
}
