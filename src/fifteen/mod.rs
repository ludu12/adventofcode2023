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


fn hash(s: &[u8]) -> usize {
    s.iter().fold(0, |curr, &b| (curr + b as usize) * 17 % 256)
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct LensBox {
    num: usize,
    lens_vec: VecDeque<(String, u8)>,
}

impl LensBox {
    fn new(n: usize) -> Self {
        Self { num: n, lens_vec: VecDeque::new() }
    }

    fn power(&self) -> u32 {
        let p: usize = self.lens_vec.iter().enumerate()
            .map(|(i, lens)| (self.num + 1) * (i + 1) * lens.1 as usize)
            .sum();

        return p as u32
    }
}

fn process(input: &str, part2: bool) -> u32 {
    let entries = input.trim().split(",").collect_vec();

    return if part2 {
        let mut boxes = (0..256).map(LensBox::new).collect::<Vec<LensBox>>();

        for entry in entries {
            let (label, f) = entry.split_once(['=', '-']).unwrap();
            let h = hash(label.as_bytes());

            // - operation
            if f.is_empty() {
                if let Some(i) = boxes[h].lens_vec.iter().position(|(l, i)| *l == label ) {
                    boxes[h].lens_vec.remove(i);
                }
            }
            // + operation
            else {
                let tuple = (label.to_string(), f.parse().unwrap());
                if let Some(i) = boxes[h].lens_vec.iter().position(|(l, i)| *l == label ) {
                    boxes[h].lens_vec[i] = tuple;
                }
                else {
                    boxes[h].lens_vec.push_back(tuple)
                }
            }
        }

        boxes.iter().filter_map(|b|
            if b.lens_vec.is_empty() {
                None
            } else {
                Some(b.power())
            }).sum()
    } else {
        entries.iter().map(|l| hash(l.as_bytes()) as u32).sum()
    };
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(145, process(input, true));
    }
}
