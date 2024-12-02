use std::str::FromStr;

use aoc24 as aoc_lib;

fn parse_report(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(i32::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn is_safe_report_impl(report: &[i32]) -> bool {
    report.is_sorted_by(|a, b| (a - b).abs() <= 3 && a < b)
        || report.is_sorted_by(|a, b| (a - b).abs() <= 3 && a > b)
}

fn is_safe_report(line: &str) -> bool {
    let report = parse_report(line);
    is_safe_report_impl(&report)
}

fn is_safe_report_dampener(line: &str) -> bool {
    let report = parse_report(line);
    if is_safe_report_impl(&report) {
        return true;
    }

    for i in 0..report.len() {
        let mut r = report.clone();
        r.remove(i);
        if is_safe_report_impl(&r) {
            return true;
        }
    }

    false
}

fn main() {
    let lines = aoc_lib::read_lines("day02.txt");

    let part_1: i32 = lines.iter().map(|l| is_safe_report(l) as i32).sum();
    println!("part_1 {part_1}");

    let part_2: i32 = lines
        .iter()
        .map(|l| is_safe_report_dampener(l) as i32)
        .sum();
    println!("part_2 {part_2}");
}
