#![allow(warnings, unused)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::panic::panic_any;
use std::pin::pin;
use itertools::Itertools;
use crate::utils::{Direction, grid, lcm, Position, print_grid};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}


fn process(input: &str, part2: bool) -> usize {
    return 1;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(1, process(input,false));
    }
}
