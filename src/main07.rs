use std::{collections::BTreeMap, env, error::Error, fs::read_to_string};

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

    let mut root: BTreeMap<String, Entry> = Default::default();
    let mut path: Vec<String> = vec![];
    let mut cwd = &mut root;

    for (i, line) in read_to_string("input07.txt")?.lines().enumerate() {
        if !line.starts_with("$ ") {
            let (size, name) = line.split(" ").collect_tuple().unwrap();
            if size == "dir" {
                cwd.insert(name.to_string(), Entry::Dir(Default::default()));
            } else {
                cwd.insert(name.to_string(), Entry::File(size.parse().unwrap()));
            }
        } else if line.starts_with("$ cd") {
            let name = &line[5..];
            if name == "/" {
                path = vec![];
                cwd = &mut root;
            } else if name == ".." {
                path.pop();
                cwd = &mut root;
                for p in &path {
                    cwd = match cwd.get_mut(p).unwrap() {
                        Entry::Dir(x) => x,
                        Entry::File(_) => panic!(),
                    }
                }
            } else {
                path.push(name.to_owned());
                cwd = match cwd.get_mut(name).unwrap() {
                    Entry::Dir(x) => x,
                    Entry::File(_) => panic!(),
                }
            }
        }
    }

    let mut acc = BTreeMap::new();
    calc_total_sizes(&root, &mut acc);
    if !part2 {
        dbg!(acc.values().filter(|x| **x < 100_000).sum::<usize>());
    } else {
        let total = acc.get(&root).unwrap();
        dbg!(acc
            .values()
            .filter(|x| **x > total - (70000000 - 30000000))
            .min());
    }

    Ok(())
}

fn calc_total_sizes<'b, 'a: 'b>(
    dir: &'a BTreeMap<String, Entry>,
    acc: &'b mut BTreeMap<&'a BTreeMap<String, Entry>, usize>,
) {
    let mut s = 0;
    for e in dir.values() {
        match e {
            Entry::Dir(d) => {
                calc_total_sizes(d, acc);
                s += acc.get(d).unwrap();
            }
            Entry::File(sz) => s += *sz,
        }
    }
    acc.insert(dir, s);
}
