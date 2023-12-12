#![allow(warnings, unused)]

use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::format;
use std::panic::panic_any;
use itertools::{Itertools, join};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}


fn f(cache: &mut HashMap<(usize, usize, usize), usize>, s: &Vec<char>, counts: &Vec<usize>, i: usize, ci: usize, current: usize) -> usize {
    let key = (i, ci, current);
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    // End of Base case
    if i == s.len() {
        return if ci == counts.len() && current == 0 {
            1
        } else if ci == counts.len() - 1 && counts[ci] == current {
            1
        } else {
            0
        }
    }
    let mut ans = 0;
    for char in ['.','#'] {
        if s[i] == char || s[i] == '?' {
            if char == '.' && current == 0 {
                // starting new count
                ans += f(cache, s, counts, i+1, ci, 0);
            }
            else if char == '.' && current > 0 && ci < counts.len() && counts[ci] == current {
                // ending new count + starting new count
                ans += f(cache, s, counts, i+1, ci+1, 0);
            }
            else if char == '#' {
                // increasing count
                ans += f(cache, s, counts, i+1, ci, current + 1);
            }
        }
    }

    cache.insert(key, ans);
    return ans;
}

fn process(input: &str, part2: bool) -> usize {
    return input.lines().map(|l| {
        let (mask, amounts) = l.split_once(" ").unwrap();

        let mut cache = HashMap::new();

        return if part2 {
            let nums = (0..5).map(|_| amounts).join(",").split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();
            let springs = (0..5).map(|_| mask).join("?").chars().collect_vec();
            f(&mut cache, &springs, &nums, 0, 0, 0)
        } else {
            let nums = amounts.split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();
            let springs = mask.chars().collect_vec();
            f(&mut cache, &springs, &nums, 0, 0, 0)
        }
    }).sum::<usize>();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        assert_eq!(1, process("? 1", false));
        // assert_eq!(2, process("?? 1", false));
    }

    #[test]
    fn part1b() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(21, process(input, false));
    }

    #[test]
    fn part1c() {
        let input = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";
        // all correct
        assert_eq!(6, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(525152, process(input, true));
    }
}
