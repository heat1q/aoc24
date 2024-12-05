use std::{cmp::Ordering, collections::BTreeSet, str::FromStr};

use aoc24 as aoc_lib;
use itertools::Itertools;

fn parse_lines(lines: &[String]) -> (BTreeSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut split = lines.split(|l| l.is_empty());
    let rules: BTreeSet<(usize, usize)> = split
        .next()
        .unwrap()
        .iter()
        .filter_map(|r| r.split_once('|'))
        .filter_map(|(l, r)| Some((l.parse().ok()?, r.parse().ok()?)))
        .collect();

    let updates = split
        .next()
        .unwrap()
        .iter()
        .map(|u| {
            u.split(',')
                .map(usize::from_str)
                .filter_map(Result::ok)
                .collect_vec()
        })
        .collect_vec();

    (rules, updates)
}

fn accumulate(_rules: &BTreeSet<(usize, usize)>, is_ordered: bool, u: Vec<usize>) -> usize {
    if is_ordered {
        u[u.len() / 2]
    } else {
        0
    }
}

fn sort_and_accumulate(
    rules: &BTreeSet<(usize, usize)>,
    is_ordered: bool,
    mut u: Vec<usize>,
) -> usize {
    if !is_ordered {
        u.sort_by(|a, b| {
            if rules.contains(&(*a, *b)) {
                Ordering::Less
            } else if rules.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        u[u.len() / 2]
    } else {
        0
    }
}

fn collect_update_numbers(
    lines: &[String],
    accumulator: impl Fn(&BTreeSet<(usize, usize)>, bool, Vec<usize>) -> usize,
) -> usize {
    let (rules, updates) = parse_lines(lines);
    updates.into_iter().fold(0usize, |acc, u| {
        let is_ordered = u.iter().enumerate().all(|(i, ival)| {
            u.iter().enumerate().all(|(j, jval)| match j.cmp(&i) {
                Ordering::Less => rules.contains(&(*jval, *ival)),
                Ordering::Greater => rules.contains(&(*ival, *jval)),
                Ordering::Equal => true,
            })
        });

        acc + accumulator(&rules, is_ordered, u)
    })
}

fn main() {
    let lines = aoc_lib::read_lines("day05.txt");

    let part_1 = collect_update_numbers(&lines, accumulate);
    println!("part_1 {part_1}");

    let part_2 = collect_update_numbers(&lines, sort_and_accumulate);
    println!("part_2 {part_2}");
}
