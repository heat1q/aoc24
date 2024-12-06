use core::str;

use aoc24 as aoc_lib;
use itertools::Itertools;

fn _print_grid(grid: &[Vec<u8>]) {
    grid.iter().for_each(|r| {
        let row = str::from_utf8(r.as_slice()).unwrap();
        println!("{row}");
    })
}

fn find_guard(grid: &[Vec<u8>]) -> Option<Point> {
    let (i, _) = grid
        .iter()
        .flat_map(|g| g.iter())
        .find_position(|g| matches!(g, b'>' | b'<' | b'^' | b'v'))?;

    Some(Point {
        x: i / grid[0].len(),
        y: i % grid[0].len(),
    })
}

#[derive(Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Line(pub Point, pub Point);

impl Line {
    pub fn contains(&self, p: &Point) -> bool {
        ((p.x <= self.0.x && p.x >= self.1.x) || (p.x >= self.0.x && p.x <= self.1.x))
            || ((p.y <= self.0.y && p.y >= self.1.y) || (p.y >= self.0.y && p.y <= self.1.y))
    }
}

fn is_cycle(lines: &[Line], p: &Point) -> bool {
    if lines.len() < 3 {
        return false;
    }

    lines
        .iter()
        .skip(lines.len() - 3)
        .rev()
        .any(|l| l.contains(p))
}

fn move_guard(grid: &mut [Vec<u8>]) -> usize {
    let mut last = find_guard(grid).unwrap();
    let mut lines: Vec<Line> = Vec::with_capacity(grid.len() * grid[0].len());
    let mut cycles = 0usize;

    loop {
        let Some(Point { x: n, y: m }) = find_guard(grid) else {
            return cycles;
        };
        let cur = Point { x: n, y: m };

        match grid[n][m] {
            b'>' => {
                for mm in m..(grid[n].len()) {
                    match grid[n][mm] {
                        b'#' => {
                            grid[n][mm - 1] = b'v';
                            lines.push(Line(last, cur.clone()));
                            break;
                        }
                        b'X' => {
                            cycles += is_cycle(&lines, &cur) as usize;
                        }
                        _ => (),
                    }

                    grid[n][mm] = b'X';
                }
            }
            b'<' => {
                for mm in (0..=m).rev() {
                    match grid[n][mm] {
                        b'#' => {
                            grid[n][mm + 1] = b'^';
                            lines.push(Line(last, cur.clone()));
                            break;
                        }
                        b'X' => {
                            cycles += is_cycle(&lines, &cur) as usize;
                        }
                        _ => (),
                    }
                    grid[n][mm] = b'X';
                }
            }
            b'v' => {
                for nn in n..(grid.len()) {
                    match grid[nn][m] {
                        b'#' => {
                            grid[nn - 1][m] = b'<';
                            lines.push(Line(last, cur.clone()));
                            break;
                        }
                        b'X' => {
                            cycles += is_cycle(&lines, &cur) as usize;
                        }
                        _ => (),
                    };
                    grid[nn][m] = b'X';
                }
            }
            b'^' => {
                for nn in (0..=n).rev() {
                    match grid[nn][m] {
                        b'#' => {
                            grid[nn + 1][m] = b'>';
                            lines.push(Line(last, cur.clone()));
                            break;
                        }
                        b'X' => {
                            cycles += is_cycle(&lines, &cur) as usize;
                        }
                        _ => (),
                    };
                    grid[nn][m] = b'X';
                }
            }
            _ => unreachable!(),
        }

        last = cur;
    }
}

fn count_positions(grid: &[Vec<u8>]) -> u64 {
    grid.iter()
        .flat_map(|g| g.iter())
        .map(|b| matches!(b, b'X') as u64)
        .sum()
}

fn main() {
    let mut grid = aoc_lib::read_lines("day06_sample.txt")
        .into_iter()
        .map(String::into_bytes)
        .collect_vec();

    let cycles = move_guard(&mut grid);
    let part_1 = count_positions(&grid);
    println!("part_1 {part_1}");

    println!("part_2 {cycles}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_contains() {
        let line = Line(Point { x: 0, y: 10 }, Point { x: 0, y: 0 });
        assert!(line.contains(&Point { x: 0, y: 10 }));
    }
}
