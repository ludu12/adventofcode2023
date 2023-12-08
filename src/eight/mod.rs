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

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    origin: String,
    left: String,
    right: String,
}

impl Node {
    pub fn new(o: &str, places: &str, part2: bool) -> Node {
        // HOW TO REGEX ? lol
        let s = places.replace(&['(', ')', ','], "");
        let (l, r) = s.split_once(' ').unwrap();

        Node {
            origin: String::from(o),
            left: String::from(l),
            right: String::from(r),
        }
    }
}


fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn traverse(start: &String, pattern: &Vec<&u8>, map: &HashMap<String, Node>) -> i64 {
    let mut entry = start;
    let mut index = 0;
    let mut step = 0;

    while !entry.ends_with("Z") {
        let direction = pattern[index];

        let node = map.get(entry).unwrap();
        entry = match direction {
            b'L' => &node.left,
            b'R' => &node.right,
            _ => unreachable!(),
        };

        index = (index + 1) % pattern.len();
        step += 1;
    }

    return step;
}


fn traverse_part2(pattern: &Vec<&u8>, map: &HashMap<String, Node>) -> i64 {
    let mut keys: Vec<&String> = map.keys().clone().filter_map(|k| {
        return if k.ends_with("A") {
            Some(k)
        } else {
            None
        };
    }).collect();

    let mut index = 0;
    let mut step = 0;
    while !keys.iter().all(|k| { k.ends_with("Z") }) {
        let direction = pattern[index];

        keys = keys.iter().map(|k| {
            let node = map.get(*k).unwrap();
            return match direction {
                b'L' => &node.left,
                b'R' => &node.right,
                _ => unreachable!(),
            };
        }).into_iter().collect_vec();

        println!("{:?}", keys);

        index = (index + 1) % pattern.len();
        step += 1;
    }

    return step;
}

fn process(input: &str, part2: bool) -> i64 {
    let (p, g) = input.split_once("\n\n").unwrap();
    let pattern: Vec<&u8> = p.as_bytes().iter().collect_vec();
    let mut node_map = HashMap::new();
    g.lines().for_each(|l| {
        let (o, d) = l.split_once(" = ").unwrap();
        node_map.insert(String::from(o), Node::new(o, d, part2));
    });

    return if part2 {
        let mut keys: Vec<&String> = node_map.keys().clone().filter_map(|k| {
            return if k.ends_with("A") {
                Some(k)
            } else {
                None
            };
        }).collect();
        let cycles = keys.iter().map(|k| traverse(*k, &pattern, &node_map)).collect_vec();

        return  cycles.iter().fold(1, |l, val| lcm(l, *val));
    } else {
        traverse(&String::from("AAA"), &pattern, &node_map)
    };
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input, false));
    }

    #[test]
    fn part1b() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input, false));
    }


    #[test]
    fn part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input, true));
    }
}
