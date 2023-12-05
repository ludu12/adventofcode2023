#![allow(warnings, unused)]

use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input);
    println!("Part1: {}", part1.to_string());
}

fn process(input: &str) -> i64 {
    let parts = input.split("\n\n");
    let collection = parts.collect::<Vec<&str>>();

    let mut seeds = collection[0].split(": ").nth(1).unwrap().split(" ").map(|x| { x.parse::<i64>().unwrap()}).collect::<Vec<i64>>();

    let mut maps = vec![];
    for i in 1..collection.len() {
        let c = collection[i];
        let mut map = vec![];

        for line in c.lines().skip(1) {
            let m = line.split(' ').map(|x| { x.parse::<i64>().unwrap()}).collect::<Vec<i64>>();

            map.push([m[0], m[1], m[2]])
        }
        maps.push(map);
    }

    let mut seed_maps = vec![];
    for seed in seeds {
        let mut seed_map = vec![];
        let mut chain = seed;
        seed_map.push(seed);

        for map in maps.iter() {
            for &[dest, src, len] in map {
                if (src..(src + len)).contains(&chain) {
                    chain = chain - src + dest;
                    break;
                }
            }
            seed_map.push(chain);
        }
        seed_maps.push(seed_map)
    }

    let mut min_location = *seed_maps[0].last().unwrap();
    for seed_map in seed_maps {
        let location = *seed_map.last().unwrap();

        if location <= min_location {
            min_location = location;
        }
    }

    return min_location;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(35, process(input));
    }
}
