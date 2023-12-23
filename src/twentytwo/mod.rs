#![allow(warnings, unused)]

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    z: usize,
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick {
    from: Coordinate,
    to: Coordinate,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        return self.from.x.max(other.from.x) <= self.to.x.min(other.to.x) &&
            self.from.y.max(other.from.y) <= self.to.y.min(other.to.y);
    }
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    return input.lines().map(|l| {
        let (from, to) = l.split_once('~').unwrap();

        let mut from_split = from.split(',');
        let from_x = from_split.next().unwrap().parse().unwrap();
        let from_y = from_split.next().unwrap().parse().unwrap();
        let from_z = from_split.next().unwrap().parse().unwrap();

        let mut to_split = to.split(',');
        let to_x = to_split.next().unwrap().parse().unwrap();
        let to_y = to_split.next().unwrap().parse().unwrap();
        let to_z = to_split.next().unwrap().parse().unwrap();

        let from = Coordinate {
            z: from_z,
            x: from_x,
            y: from_y,
        };
        let to = Coordinate {
            z: to_z,
            x: to_x,
            y: to_y,
        };
        return if from > to {
            Brick { from: to, to: from }
        } else {
            Brick { from, to }
        };
    }).collect_vec();
}

fn drop_bricks(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut new_bricks = Vec::new();

    for i in (0..bricks.len()) {
        let brick = bricks[i];
        let mut bottom = 1;

        // Loop through all the placed bricks
        for j in (0..new_bricks.len()) {
            let to_check = new_bricks[j];
            // find any brick below this one that overlaps xy axis and return that value
            if (brick.overlaps(&to_check)) {
                bottom = bottom.max(to_check.to.z + 1);
            }
        }

        let diff = brick.to.z - brick.from.z;
        new_bricks.push(Brick {
            from: Coordinate {
                x: brick.from.x,
                y: brick.from.y,
                z: bottom,
            },
            to: Coordinate {
                x: brick.to.x,
                y: brick.to.y,
                z: bottom + (diff),
            },
        });
    }

    new_bricks.sort_by_key(|b| b.from);
    return new_bricks;
}

fn process(input: &str, part2: bool) -> usize {
    let mut bricks = parse_bricks(input);
    bricks.sort_by_key(|b| b.from);
    let dropped_bricks = drop_bricks(&bricks);

    // Using indices for set
    let mut supports = vec![HashSet::new(); dropped_bricks.len()];
    let mut supported_by = vec![HashSet::new(); dropped_bricks.len()];

    for i in (0..dropped_bricks.len()) {
        let upper = dropped_bricks[i];
        for j in (0..i) {
            let lower = dropped_bricks[j];

            // if they overlap and the upper is sitting on top of the lower
            if lower.overlaps(&upper) && upper.from.z == lower.to.z + 1 {
                supported_by[i].insert(j);
                supports[j].insert(i);
            }
        }
    }


    let total = (0..dropped_bricks.len()).fold(0, |total, i| {
        // if the bricks this one supports has 2 or more other supporters, then we can remove
        if (supports[i].iter().all(|b| {
            let x = supported_by.iter().nth(*b).unwrap().len() >= 2;



            return x;
        })) {
            return total + 1;
        }
        total
    });

    return total;
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
        assert_eq!(5, process(input, false));
    }
}
