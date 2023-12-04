#![allow(warnings, unused)]

use std::cmp::min;
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    // let part2 = process(input, true);
    // println!("Part2: {}", part2.to_string());
}

fn intersection(nums: Vec<Vec<u32>>) -> Vec<u32> {
    let mut intersect_result: Vec<u32> = nums[0].clone();

    for temp_vec in nums {
        let unique_a: HashSet<u32> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }
    return intersect_result
}

fn process(input: &str, part2: bool) -> u32 {
    return input
        .lines()
        .map(|line| {
            let (i, d) = line.split_once(": ").unwrap();
            let id = i.split(" ").last()
                .expect("card should have an id")
                .parse::<u32>()
                .expect("card id should be a valid digit");

            let (w, m) = d.split_once(" | ").unwrap();
            let mut winning: Vec<u32> = w
                .split_whitespace()
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect();
            winning.dedup();

            let mine: Vec<u32> = m
                .split_whitespace()
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect();

            let intersection = intersection(vec![winning, mine]);

            let len: u32 = intersection.len() as u32;
            if len > 0 {
                return 2_u32.pow(len - 1);
            }
            return 0;
        }).sum::<u32>();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input, true));
    }
}
