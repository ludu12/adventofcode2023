#![allow(warnings, unused)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::panic::panic_any;
use itertools::Itertools;
use crate::utils::print_grid;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> u32 {
    for line in input.lines() {
        let (module, name) = line.split_once(" -> ").unwrap();
        println!("{module}");
    };


    return 1;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(1, process(input, false));
    }

}
