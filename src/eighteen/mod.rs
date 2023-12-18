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
    meters: u8,

    // hex: &
}

fn calc_area(instructions: &Vec<Instruction>) -> i64 {
    return 1;
}

fn process(input: &str, part2: bool) -> u32 {
    let instructions = input.lines().map(|l| {
        let (code, hex) = l.split_once(" (#").unwrap();

        let bytes = code.as_bytes();
        let l = bytes[2] - b'0';
        let d = match bytes[0] {
            b'U' => Direction::Up,
            b'R' => Direction::Right,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            _ => unreachable!()
        };
        return Instruction {
            direction: d,
            meters: l
        }
    }).collect_vec();



    return 62;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(145, process(input, true));
    }
}
