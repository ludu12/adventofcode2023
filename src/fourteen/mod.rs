#![allow(warnings, unused)]

use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::format;
use std::panic::panic_any;
use itertools::{Itertools, join};
use crate::utils::{grid, transpose};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}


fn calc(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    assert_eq!(rows, cols); // theres sort of an assumption here that this is a n x n grid

    let mut answer = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'O' {
                answer += rows - r;
            }
        }
    }

    answer
}

fn tilt(grid: &mut Vec<Vec<char>>) {
    let rows_len = grid.len();
    let cols_len = grid[0].len();
    assert_eq!(rows_len, cols_len); // theres sort of an assumption here that this is a n x n grid

    for c in (0..cols_len) {
        let mut index = 0;

        for r in (0..rows_len) {
            match grid[r][c] {
                '#' => {
                    index = r;
                }
                'O' => {
                    while index < r && grid[index][c] != '.' {
                        index += 1;
                    }

                    if index != r {
                        let x = grid[index][c];
                        grid[index][c] = grid[r][c];
                        grid[r][c] = x;
                    }
                }
                _ => {}
            }
        }
    }
}

fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            new_grid[c][grid.len() - 1 - r] = grid[r][c];
        }
    }
    new_grid
}

fn process(input: &str, part2: bool) -> u32 {
    let mut g = grid(input);

    if part2 {
        let mut seen = HashMap::new();
        for i in 1..1000000000 {
            for _ in 0..4 {
                tilt(&mut g);
                g = rotate(&g);
            }
            if let Some(seen_at) = seen.insert(g.clone(), i) {
                if (1000000000 - i) % (i - seen_at) == 0 {
                    break;
                }
            }
        }
    } else {
        tilt(&mut g);
    }

    let t = calc(&g);
    return t as u32;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = "O.O.
O#O.
.O.O
....";
        assert_eq!(20, process(input, false));
    }

    #[test]
    fn part1b() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(136, process(input, false));
    }

    #[test]
    fn part2a() {
        let input = "O.O.
O#O.
.O.O
....";
        assert_eq!(20, process(input, false));
    }

    #[test]
    fn part2b() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(64, process(input, true));
    }
}
