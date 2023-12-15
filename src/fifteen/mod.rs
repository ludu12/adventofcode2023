#![allow(warnings, unused)]

use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::format;
use std::panic::panic_any;
use itertools::{Itertools, join};
use log::debug;
use crate::utils::{grid, transpose};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}


fn hash(s: &[u8]) -> u32 {
    s.iter().fold(0, |curr, &b| (curr + b as u32) * 17 % 256)
}

fn process(input: &str, part2: bool) -> u32 {
    return input.trim().split(",").map(|l| hash(l.as_bytes())).sum();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, process(input, false));
    }
}
