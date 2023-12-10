#![allow(warnings, unused)]

use std::collections::{HashMap};
use itertools::Itertools;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn next_sum(vec: &Vec<i64>) -> i64 {
    if vec.iter().all(|i| *i == 0) {
        return 0;
    }

    let mut new_vec: Vec<i64> = vec![];
    for i in 0..vec.len() - 1 {
        new_vec.push(vec[i + 1] - vec[i]);
    }

    return new_vec.last().unwrap() + next_sum(&new_vec);
}

fn first_sum(vec: &Vec<i64>) -> i64 {
    if vec.iter().all(|i| *i == 0) {
        return 0;
    }

    let mut new_vec: Vec<i64> = vec![];
    for i in 0..vec.len() - 1 {
        new_vec.push(vec[i + 1] - vec[i]);
    }

    return new_vec.first().unwrap() - first_sum(&new_vec);
}

fn process(input: &str, part2: bool) -> i64 {
    let arr = input.lines().map(|l| {
        let nums = l.split(" ").map(|x| x.parse::<i64>().unwrap()).collect_vec();

        if part2 {
            return nums.first().unwrap() - first_sum(&nums);
        }
        else {
            return nums.last().unwrap() + next_sum(&nums);
        }
    }).sum::<i64>();

    return arr;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input, true));
    }
}
