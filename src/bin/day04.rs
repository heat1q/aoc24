use aoc24 as aoc_lib;
use itertools::Itertools;

const DIM: [[i64; 2]; 8] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
    [1, 1],
    [-1, 1],
    [1, -1],
    [-1, -1],
];

fn count_xmas(grid: &[Vec<u8>], n: usize, m: usize) -> usize {
    DIM.iter()
        .map(|[x, y]| {
            let buf = (0..4)
                .map(|i| {
                    let nx: usize = ((n as i64) + i * x) as usize;
                    let my: usize = ((m as i64) + i * y) as usize;
                    grid.get(nx)
                        .and_then(|g| g.get(my))
                        .cloned()
                        .unwrap_or_default()
                })
                .collect_vec();

            matches!(buf.as_slice(), b"XMAS" | b"SAMX") as usize
        })
        .sum()
}

const DIAG: [[i64; 2]; 4] = [[1, 1], [-1, 1], [1, -1], [-1, -1]];

fn count_x_mas(grid: &[Vec<u8>], n: usize, m: usize) -> usize {
    DIAG.iter().all(|[x, y]| {
        let buf = (-1..2)
            .map(|i| {
                let nx: usize = ((n as i64) + i * x) as usize;
                let my: usize = ((m as i64) + i * y) as usize;
                grid.get(nx)
                    .and_then(|g| g.get(my))
                    .cloned()
                    .unwrap_or_default()
            })
            .collect_vec();

        matches!(buf.as_slice(), b"MAS" | b"SAM")
    }) as usize
}

fn part_1(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .flat_map(|g| g.iter())
        .enumerate()
        .map(|(i, val)| {
            let n = i / grid.len();
            let m = i % grid.len();
            if val == &b'X' {
                count_xmas(grid, n, m)
            } else {
                0
            }
        })
        .sum()
}

fn part_2(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .flat_map(|g| g.iter())
        .enumerate()
        .map(|(i, val)| {
            let n = i / grid.len();
            let m = i % grid.len();
            if val == &b'A' {
                count_x_mas(grid, n, m)
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let grid = aoc_lib::read_lines("day04.txt")
        .into_iter()
        .map(String::into_bytes)
        .collect_vec();

    let part_1 = part_1(&grid);
    println!("part_1 {part_1}");

    let part_2 = part_2(&grid);
    println!("part_2 {part_2}");
}
