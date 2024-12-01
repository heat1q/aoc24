use std::{collections::BTreeMap, iter::zip};

use aoc24 as aoc_lib;

fn unzip_list(lines: &[String]) -> (Vec<i32>, Vec<i32>) {
    lines
        .iter()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(l, r)| Some((l.trim().parse::<i32>().ok()?, r.trim().parse::<i32>().ok()?)))
        .unzip()
}

fn count_difference(lines: &[String]) -> i32 {
    let (mut left, mut right) = unzip_list(lines);
    left.sort();
    right.sort();

    zip(left, right).map(|(l, r)| (l - r).abs()).sum()
}

fn similiarity_score(lines: &[String]) -> i32 {
    let (left, right) = unzip_list(lines);
    let mut rr: BTreeMap<i32, i32> = BTreeMap::new();
    for r in &right {
        rr.entry(*r).and_modify(|e| *e += 1).or_insert(1);
    }

    left.into_iter()
        .map(|l| rr.get(&l).cloned().unwrap_or_default() * l)
        .sum()
}

fn main() {
    let lines = aoc_lib::read_lines("day01.txt");

    let part_1 = count_difference(&lines);
    println!("part_1 {part_1}");

    let part_2 = similiarity_score(&lines);
    println!("part_2 {part_2}");
}
