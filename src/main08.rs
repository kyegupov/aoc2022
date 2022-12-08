use std::{cmp::max, collections::BTreeMap, env, error::Error, fs::read_to_string};

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

    let mut matrix: Vec<Vec<u64>> = vec![];

    for (i, line) in read_to_string("input08.txt")?.lines().enumerate() {
        matrix.push(
            line.chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect(),
        );
    }

    let hei = matrix.len();
    let wid = matrix[0].len();

    if !part2 {
        let mut visible = 0;
        for (j, row) in matrix.iter().enumerate() {
            for (i, &cell) in row.iter().enumerate() {
                if (0..i).all(|ii| matrix[j][ii] < cell)
                    || (i + 1..wid).all(|ii| matrix[j][ii] < cell)
                    || (0..j).all(|jj| matrix[jj][i] < cell)
                    || (j + 1..hei).all(|jj| matrix[jj][i] < cell)
                {
                    visible += 1;
                }
            }
        }
        dbg!(visible);
    } else {
        let mut max_scenic = 0;
        for (j, row) in matrix.iter().enumerate() {
            for (i, &cell) in row.iter().enumerate() {
                let ranges = (
                    (0..i)
                        .rev()
                        .position(|ii| matrix[j][ii] >= cell)
                        .map(|x| x + 1)
                        .unwrap_or(i),
                    (i + 1..wid)
                        .position(|ii| matrix[j][ii] >= cell)
                        .map(|x| x + 1)
                        .unwrap_or(wid - (i + 1)),
                    (0..j)
                        .rev()
                        .position(|jj| matrix[jj][i] >= cell)
                        .map(|x| x + 1)
                        .unwrap_or(j),
                    (j + 1..hei)
                        .position(|jj| matrix[jj][i] >= cell)
                        .map(|x| x + 1)
                        .unwrap_or(hei - (j + 1)),
                );
                max_scenic = max(max_scenic, ranges.0 * ranges.1 * ranges.2 * ranges.3);
            }
        }
        dbg!(max_scenic);
    }

    Ok(())
}
