use std::{
    cmp::min,
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs::read_to_string,
    iter,
    rc::Rc,
};

use bit_set::BitSet;
use nom::{
    bytes::complete::{tag, take_while},
    character::{complete::alpha1, complete::digit1},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, tuple},
    AsChar, IResult,
};

#[derive(Debug)]
struct Rules {
    valves: BTreeSet<Rc<String>>,

    flows: BTreeMap<Rc<String>, u64>, // only valves with nonzero flows
    distances: BTreeMap<Rc<String>, BTreeMap<Rc<String>, u64>>, // distance between valves

    // Same as above, but using indices of valves instead of names.
    // idx 0 = start
    // other idx are indexes in flows + 1
    flows_by_idx: Vec<u64>,
    distances_by_idx: Vec<Vec<u64>>,

    part2: bool,
    start: Rc<String>,
}

struct SolutionState<'a> {
    minutes_left: u64,
    location: usize,
    current_flow: u64,
    total_flow: u64,
    is_final_stage: bool, // true if part 1 or elephant in part 2
    opened: &'a mut BitSet,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Persisted {
    global_best: u64,
    memoized_elephant_solutions_by_preopened_valves: BTreeMap<BitSet, u64>,
}

impl Persisted {
    fn try_solution_step(&mut self, s: SolutionState, r: &Rules) -> u64 {
        let distances = &r.distances_by_idx[s.location];
        // do-nothing solution
        let mut best = s.total_flow + s.current_flow * s.minutes_left;
        let mut cannot_progress = true;
        for x in 0..r.flows_by_idx.len() {
            if s.opened.contains(x) {
                continue;
            }
            let dist = distances[x];
            if dist < s.minutes_left {
                cannot_progress = false;
                let ss = SolutionState {
                    minutes_left: s.minutes_left - dist - 1,
                    total_flow: s.total_flow + (dist + 1) * s.current_flow,
                    current_flow: s.current_flow + r.flows_by_idx[x],
                    location: x.clone(),
                    is_final_stage: s.is_final_stage,
                    opened: s.opened,
                };
                ss.opened.insert(x.clone());
                let solution = self.try_solution_step(ss, r);
                if solution > best {
                    best = solution;
                }
                s.opened.remove(x);
            }
        }

        if s.is_final_stage {
            if !r.part2 && best > self.global_best {
                self.global_best = best;
            }
            return best;
        } else if cannot_progress {
            let maybe_best_elephant_solution = self
                .memoized_elephant_solutions_by_preopened_valves
                .get(&s.opened)
                .copied();
            let best_elephant_solution = maybe_best_elephant_solution.or_else(|| {
                let theor_max = (1..r.flows_by_idx.len())
                    .filter(|(k)| !s.opened.contains(*k))
                    .map(|(k)| r.flows_by_idx[k] * (26 - 1 - r.distances_by_idx[0][k]))
                    .sum::<u64>();
                if theor_max + best > self.global_best {
                    let solution_elephant = SolutionState {
                        minutes_left: 26,
                        location: 0,
                        current_flow: 0,
                        total_flow: 0,
                        is_final_stage: true,
                        opened: s.opened,
                    };
                    let best_elephant = self.try_solution_step(solution_elephant, r);
                    // dbg!(best_elephant);
                    self.memoized_elephant_solutions_by_preopened_valves
                        .insert(s.opened.clone(), best_elephant);
                    Some(best_elephant)
                } else {
                    None
                }
            });
            if let Some(best_elephant) = best_elephant_solution {
                if best + best_elephant > self.global_best {
                    self.global_best = best + best_elephant;
                    // dbg!(self.global_best);
                }
            }
            self.global_best
        } else {
            best
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
        flows_by_idx: Default::default(),
        distances_by_idx: Default::default(),
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

    rules.flows_by_idx.push(0);
    for a in rules.flows.values() {
        rules.flows_by_idx.push(*a);
    }
    for (i1, n1) in iter::once(&rules.start)
        .chain(rules.flows.keys())
        .enumerate()
    {
        let mut row = vec![];
        for (i2, n2) in iter::once(&rules.start)
            .chain(rules.flows.keys())
            .enumerate()
        {
            row.push(rules.distances[n1][n2])
        }
        rules.distances_by_idx.push(row);
    }

    let mut persisted = Persisted {
        global_best: 0,
        memoized_elephant_solutions_by_preopened_valves: BTreeMap::new(),
    };
    let state = SolutionState {
        minutes_left: if rules.part2 { 26 } else { 30 },
        location: 0,
        current_flow: 0,
        total_flow: 0,
        is_final_stage: !rules.part2,
        opened: &mut BitSet::new(),
    };
    let best = persisted.try_solution_step(state, &rules);
    dbg!(best);

    Ok(())
}
