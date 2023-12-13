#![allow(warnings, unused)]

use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::format;
use std::panic::panic_any;
use itertools::{Itertools, izip, join};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    // 37982
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn to_bit_mask_int(v: &Vec<char>) -> u64 {
    return v.into_iter()
        .map(|c| if *c == '#' { 1 } else { 0 })
        .rev().enumerate()
        .map(|(place, bit)| bit << place)
        .sum();
}

fn compare(rows: &Vec<Vec<char>>, x1: usize, x2: usize) -> u64 {
    let r1 = to_bit_mask_int(rows.iter().nth(x1).unwrap());
    let r2 = to_bit_mask_int(rows.iter().nth(x2).unwrap());
    return (r1 ^ r2).count_ones() as u64;
}

fn calc(rows: &Vec<Vec<char>>, part2: bool) -> u64 {
    for row in 1..rows.len() {
        let mut u = row as i32 - 1;
        let mut d = row;
        let mut errors = 0;

        while u >= 0 && d < rows.len() {
            if part2 && errors > 1 {
                break;
            } else if !part2 && errors > 0 {
                break;
            }

            errors += compare(rows, u as usize, d);
            u -= 1;
            d += 1;
        }

        if part2 && errors == 1 {
            return row as u64;
        } else if !part2 && errors == 0 {
            return row as u64;
        }
    }

    return 0;
}

fn process(input: &str, part2: bool) -> u64 {
    let patterns = input.split("\n\n")
        .map(|x| x.lines()
            .map(|l| l.chars()
                .collect_vec())
            .collect_vec())
        .collect_vec();

    return patterns.iter().map(|pattern| {
        let horizontal = pattern.clone();
        let vertical = transpose(pattern.clone());

        let v = calc(&vertical, part2);
        return if v != 0 {
            v
        } else {
            let h = calc(&horizontal, part2);
            100 * h
        }
    }).sum();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_mask() {
        assert_eq!(0, to_bit_mask_int(&vec!['.']));
        assert_eq!(1, to_bit_mask_int(&vec!['#']));
        assert_eq!(2, to_bit_mask_int(&vec!['#', '.']));
        assert_eq!(3, to_bit_mask_int(&vec!['#', '#']));

        // "#.##..##."
        // 101100110
        // 358
        assert_eq!(358u32.count_ones(), 5);
        assert_eq!(358, to_bit_mask_int(&"#.##..##.".chars().collect_vec()));

    }

    #[test]
    fn compare_bit_maske() {
        let rows = vec![
            "...#.#####....#".chars().collect_vec(),
            "..##.#####....#".chars().collect_vec()
        ];
        assert_eq!(compare(&rows, 0, 1), 1);
    }


    #[test]
    fn part1a() {
        let mut input = "..#
.##
###
###
.##
..#
...";
        assert_eq!(300, process(input, false));

        input = "..##..
.####.
######
.####.
..##..";
        assert_eq!(3, process(input, false));

        input = "..#
.##
###
###
.##
#.#
...";
        assert_eq!(0, process(input, false));
    }

    #[test]
    fn part1b() {
        let mut input = "..#
.##
###
###
.##
..#
...";
        assert_eq!(300, process(input, false));

        input = "..##..
.####.
######
.####.
..##..";
        assert_eq!(3, process(input, false));

        input = "..#
.##
###
###
.##
#.#
...";
        assert_eq!(0, process(input, false));
    }

    #[test]
    fn part1c() {
        let mut input =
            "#.#..##..#.
#...#..#...
..###..###.
#..##..##..
.....##.#..
##........#
..##.##.##.
.##......##
.##......##";
        assert_eq!(800, process(input, false));
    }

    #[test]
    fn part1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(405, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(405, process(input, true));
    }

    #[test]
    fn part2b() {
        let input = "##..#...#.#..#.
##.###...###..#
...#.#####....#
..##.#####....#
##.###...###..#
##..#...#.#..#.
..#..###.#####.";
        assert_eq!(300, process(input, true));
    }
}
