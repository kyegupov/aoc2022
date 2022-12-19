use std::{
    cmp::min,
    collections::{BTreeMap, BTreeSet, VecDeque},
    env,
    error::Error,
    fs::read_to_string,
    rc::Rc,
};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while},
    character::{complete::alpha1, complete::digit1, is_digit},
    combinator::{map, map_res, opt, recognize},
    error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    AsChar, IResult,
};

#[derive(Debug)]
struct Valve {
    flow: u64,
    routes: BTreeSet<Rc<String>>,
}

#[derive(Debug)]
struct Rules {
    valves: BTreeSet<Rc<String>>,
    flows: BTreeMap<Rc<String>, u64>, // only nonzero
    distances: BTreeMap<Rc<String>, BTreeMap<Rc<String>, u64>>,
    part2: bool,
    start: Rc<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State<'a> {
    minutes_left: u64,
    solution: Vec<Rc<String>>,
    opened: &'a mut BTreeSet<Rc<String>>,
    current_flow: u64,
    total_flow: u64,
    best: u64,
    memoized_elephant_solutions: BTreeMap<BTreeSet<Rc<String>>, u64>,
}

enum Action {
    Open,
    Move(Rc<String>),
}

impl<'a> State<'a> {
    fn find_solution(&mut self, r: &Rules, is_final: bool) {
        let location = self.solution.last().unwrap_or(&r.start);
        let distances = &r.distances[location];
        for x in r.flows.keys() {
            if self.opened.contains(x) {
                continue;
            }
            let dist = distances[x];
            // dbg!(dist, self.minutes_left);
            if dist < self.minutes_left {
                self.minutes_left -= dist + 1;
                self.total_flow += (dist + 1) * self.current_flow;
                self.current_flow += r.flows[x];
                self.solution.push(x.clone());
                self.opened.insert(x.clone());
                self.find_solution(r, is_final);
                self.minutes_left += dist + 1;
                self.current_flow -= r.flows[x];
                self.total_flow -= (dist + 1) * self.current_flow;
                self.solution.pop();
                self.opened.remove(x);
            }
        }
        let total_flow = self.total_flow + self.current_flow * self.minutes_left;
        if is_final {
            if total_flow > self.best {
                self.best = total_flow;
            }
        } else {
            let maybe_best_elephant_solution =
                self.memoized_elephant_solutions.get(&self.opened).copied();
            let best_elephant_solution = maybe_best_elephant_solution.or_else(|| {
                let theor_max = r
                    .flows
                    .iter()
                    .filter(|(k, v)| !self.opened.contains(*k))
                    .map(|(k, v)| v * (26 - 1 - r.distances[&r.start][k]))
                    .sum::<u64>();
                if theor_max + total_flow > self.best {
                    let mut state_inner = State {
                        minutes_left: 26,
                        solution: vec![],
                        opened: self.opened,
                        current_flow: 0,
                        total_flow: 0,
                        best: 0,
                        memoized_elephant_solutions: BTreeMap::new(),
                    };
                    state_inner.find_solution(r, true);
                    self.memoized_elephant_solutions
                        .insert(state_inner.opened.clone(), state_inner.best);
                    Some(state_inner.best)
                } else {
                    None
                }
            });
            if let Some(bes) = best_elephant_solution {
                if total_flow + bes > self.best {
                    self.best = total_flow + bes;
                    dbg!(self.best);
                }
            }
        }
    }
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(
        take_while(|x: char| x == '-' || x.is_dec_digit()),
        str::parse,
    )(input)
}

fn parse_line(x: &str) -> IResult<&str, (&str, u64, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), parse_u64),
        preceded(
            tuple((
                tag("; "),
                opt(take_while(|c: char| c.is_lowercase() || c == ' ')), // "tunnel(s) lead(s) to valve(s)", ugh
            )),
            separated_list1(tag(", "), alpha1),
        ),
    ))(x)
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut rules: Rules = Rules {
        valves: Default::default(),
        part2,
        flows: Default::default(),
        distances: Default::default(),
        start: Rc::new("AA".to_string()),
    };

    for (y, line) in read_to_string("input16.txt")?.lines().enumerate() {
        let (s_name, flow, routes) = parse_line(line).unwrap().1;
        let name = Rc::new(s_name.to_owned());
        if flow > 0 {
            rules.flows.insert(name.clone(), flow);
        }
        for r in routes {
            rules
                .distances
                .entry(name.clone())
                .or_default()
                .insert(Rc::new(r.to_string()), 1);
        }
        rules.valves.insert(name);
    }

    for b in rules.valves.iter() {
        for a in rules.valves.iter() {
            for c in rules.valves.iter() {
                if rules.distances[a].contains_key(b) && rules.distances[b].contains_key(c) {
                    let candidate = rules.distances[a][b] + rules.distances[b][c];
                    rules
                        .distances
                        .get_mut(a)
                        .unwrap()
                        .entry(c.clone())
                        .and_modify(|x| *x = min(*x, candidate))
                        .or_insert(candidate);
                }
            }
        }
    }
    let mut opened = Default::default();
    let mut state = State {
        minutes_left: if rules.part2 { 26 } else { 30 },
        solution: vec![],
        opened: &mut opened,
        current_flow: 0,
        total_flow: 0,
        best: 0,
        memoized_elephant_solutions: BTreeMap::new(),
    };
    state.find_solution(&rules, !rules.part2);
    dbg!(state.best);

    Ok(())
}
