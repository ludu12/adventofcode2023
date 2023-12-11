#![allow(warnings, unused)]

use std::cmp::min;
use std::cmp::max;
use std::fmt::format;
use std::panic::panic_any;
use itertools::{Itertools};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> u64 {
    let multiplier = if part2 { 1000000 } else { 2 };

    // Assume there are no galaxies, double every row/column
    let mut rows: Vec<usize> = vec![multiplier; input.lines().count()];
    let mut columns: Vec<usize> = vec![multiplier; input.lines().nth(0).unwrap().len()];

    let mut galaxies: Vec<(usize, usize)> = vec![];

    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                rows[i] = 1;
                columns[j] = 1;
                galaxies.push((i, j));
            }
        })
    });

    let mut sum: u64 = 0;
    for i in 0..galaxies.len() {
        let g1 = galaxies[i];

        for j in i..galaxies.len() {
            if i != j {
                let g2 = galaxies[j];

                let y_1 = g1.0;
                let y_2 = g2.0;
                let x_1 = g1.1;
                let x_2 = g2.1;

                let y: u64 = (min(y_1, y_2)..max(y_1, y_2)).sorted().map(|y_index| { rows[y_index] as u64 }).sum();
                let x: u64 = (min(x_1, x_2)..max(x_1, x_2)).sorted().map(|x_index| { columns[x_index] as u64 }).sum();
                let distance = x + y;

                sum += distance;
            }
        }
    }

    return sum;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(82000210, process(input, true));
    }
}
