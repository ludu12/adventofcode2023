#![allow(warnings, unused)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::panic::panic_any;
use std::pin::pin;
use itertools::Itertools;
use crate::utils::{Direction, grid, lcm, Position, print_grid};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, 64, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, 64, true);
    println!("Part2: {}", part2.to_string());
}


struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(i: &str) -> Grid {
        Grid {
            grid: grid(i)
        }
    }

    fn x_len(&self) -> usize {
        self.grid[0].len()
    }

    fn y_len(&self) -> usize {
        self.grid.len()
    }

    fn at(&self, p: Position) -> char {
        if p.is_valid(self.x_len(), self.y_len()){
            return self.grid[p.y as usize][p.x as usize];
        }
        unreachable!();
    }

    fn neighbors(&self, p: Position) -> Vec<Position> {
        let v = vec![Direction::North, Direction::East, Direction::West, Direction::South];

        v.iter().filter_map(|&d| {
            let position = p.go(d);
            if self.at(position) != '#' {
                return Some(position);
            }

            return None;
        }).collect()
    }
}


fn process(input: &str, steps: usize, part2: bool) -> usize {
    let g = Grid::new(input);

    let mut queue = HashSet::new();
    for y in 0..g.y_len() {
        for x in 0..g.x_len() {
            let curr = Position { x: x as i32, y: y as i32 };
            if g.at(curr) == 'S' {
                queue.insert(curr);
            }
        }
    }
    let mut next = HashSet::new();

    for i in 0..steps {
        for p in queue.drain() {
            for n in g.neighbors(p) {
                next.insert(n);
            }
        }
        queue = next.clone();
        next.clear();
    }

    // for y in 0..g.y_len() {
    //     for x in 0..g.x_len() {
    //         let curr = Position { x: x as i32, y: y as i32 };
    //         if queue.contains(&curr) {
    //             print!("O");
    //         } else {
    //             print!("{}", g.at(curr));
    //         }
    //     }
    //     println!();
    // }
    return queue.len();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(16, process(input, 6, false));
    }
}
