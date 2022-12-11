use std::{env, error::Error, fs::read_to_string};

use itertools::Itertools;

#[derive(Default, Debug)]
struct State {
    items_per_monkey: Vec<Vec<u64>>,
    counters_per_monkey: Vec<usize>,
}

impl State {
    fn do_tick(&mut self, rules: &[Rule], part2: bool) {
        let all_divisors: u64 = rules.iter().map(|rule| rule.divisible_by).product();
        for i in 0..self.items_per_monkey.len() {
            let items = self.items_per_monkey[i].clone();
            self.items_per_monkey[i].clear();
            for item in items {
                self.counters_per_monkey[i] += 1;
                let mut item_processed = rules[i].apply(item);
                if !part2 {
                    item_processed /= 3;
                }
                item_processed = item_processed % all_divisors;
                if item_processed % rules[i].divisible_by == 0 {
                    self.items_per_monkey[rules[i].monkey_t].push(item_processed);
                } else {
                    self.items_per_monkey[rules[i].monkey_f].push(item_processed);
                }
            }
        }
    }
}

enum Operand {
    Num(u64),
    Old,
}

enum Op {
    Add,
    Mul,
}
impl Op {
    fn parse(s: &str) -> (Op, Operand) {
        let (op, operand) = s.split_once(" ").unwrap();
        (
            match op {
                "+" => Op::Add,
                "*" => Op::Mul,
                _ => panic!("{}", op),
            },
            match operand {
                "old" => Operand::Old,
                _ => Operand::Num(operand.parse().unwrap()),
            },
        )
    }
}

struct Rule {
    op: (Op, Operand),
    divisible_by: u64,
    monkey_t: usize,
    monkey_f: usize,
}
impl Rule {
    fn apply(&self, item: u64) -> u64 {
        let operand = match self.op.1 {
            Operand::Num(x) => x,
            Operand::Old => item,
        };
        match self.op.0 {
            Op::Add => item.checked_add(operand).unwrap(),
            Op::Mul => item.checked_mul(operand).unwrap(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut state = State {
        ..Default::default()
    };
    let mut rules = vec![];
    for chunk in &read_to_string("input11.txt")?.lines().chunks(7) {
        let (_, ln_items, ln_op, ln_divisible, ln_true, ln_false) =
            chunk.take(6).collect_tuple().unwrap();
        state.items_per_monkey.push(
            ln_items
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect(),
        );
        state.counters_per_monkey.push(0);
        rules.push(Rule {
            op: Op::parse(ln_op.split_once("= old ").unwrap().1),
            divisible_by: ln_divisible.split_once(" by ").unwrap().1.parse().unwrap(),
            monkey_t: ln_true.split_once("monkey ").unwrap().1.parse().unwrap(),
            monkey_f: ln_false.split_once("monkey ").unwrap().1.parse().unwrap(),
        })
    }
    let rounds = if part2 { 10000 } else { 20 };
    for _ in 0..rounds {
        state.do_tick(&rules, part2);
    }

    dbg!(&state
        .counters_per_monkey
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product::<usize>());

    Ok(())
}
