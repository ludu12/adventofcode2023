#![allow(warnings, unused)]

use std::cmp::min;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
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

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Instruction {
    direction: Direction,
    meters: i64,
    // hex: &
}

fn calc_area(instructions: &Vec<Instruction>) -> i64 {
    let mut area = 0;
    let mut perimeter = 0;
    let (mut x, mut y) = (0,0);

    for instruction in instructions {
        let m = instruction.meters;
        let (x0, y0) = (x, y);
        let length = m;
        match instruction.direction {
            Direction::Up => y += length,
            Direction::Right => x += length,
            Direction::Down => y -= length,
            Direction::Left => x -= length,
        };

        // Shoelace formula
        area += (x0 * y) - (y0 * x);

        perimeter += length;
    }

    let area = i64::abs(area) / 2;
    // Pick's theorem -> A = i + b/2 + 1
    let interior: i64 = area - (perimeter / 2) + 1;

    return interior + perimeter;
}

fn process(input: &str, part2: bool) -> i64 {
    let instructions = input.lines().map(|l| {
        let (code, hex) = l.split_once(" (#").unwrap();

        return if part2 {
            let d = hex.as_bytes()[hex.len() - 2];
            let direction = match d {
                b'0' => Direction::Up,
                b'1' => Direction::Right,
                b'2' => Direction::Down,
                b'3' => Direction::Left,
                _ => unreachable!()
            };

            let meters  = i64::from_str_radix(&hex[0..hex.len()-2], 16).unwrap();
            Instruction {
                direction,
                meters,
            }
        } else {
            let (d, l) = code.split_once(" ").unwrap();
            let meters = l.parse::<i64>().unwrap();
            let direction = match d.as_bytes()[0] {
                b'U' => Direction::Up,
                b'R' => Direction::Right,
                b'D' => Direction::Down,
                b'L' => Direction::Left,
                _ => unreachable!()
            };
            Instruction {
                direction,
                meters,
            }
        }
    }).collect_vec();

    return calc_area(&instructions);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = "R 4 (#70c710)
D 4 (#0dc571)
L 4 (#5713f0)
U 4 (#d2c081)";
        // 4 * 4 = 16 (outside)
        // 3 * 3 = 9 (inside)
        assert_eq!(25, process(input, false));
    }

    #[test]
    fn part1b() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(62, process(input, false));
    }


    #[test]
    fn part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(952408144115, process(input, true));
    }
}
