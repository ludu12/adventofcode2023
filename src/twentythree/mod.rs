#![allow(warnings, unused)]

use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::utils::{Direction, grid, Position};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

struct Grid {
    grid: Vec<Vec<char>>,
    part2: bool,
}

impl Grid {
    fn new(i: &str, part2: bool) -> Grid {
        Grid {
            grid: grid(i),
            part2
        }
    }

    fn x_len(&self) -> usize {
        self.grid[0].len()
    }

    fn y_len(&self) -> usize {
        self.grid.len()
    }

    fn end(&self) -> Position {
        // This is a known position
        Position {
            x: (self.x_len() as i32) - 2,
            y: (self.y_len() as i32) - 1,
        }
    }

    fn start(&self) -> Position {
        // This is a known position
        Position {
            x: 1,
            y: 0,
        }
    }

    fn at(&self, p: Position) -> Option<char> {
        if p.is_valid(self.x_len(), self.y_len()) {
            return Some(self.grid[p.y as usize][p.x as usize]);
        }
        return None;
    }

    fn print(&self, seen: &HashSet<Position>) {
        for y in 0..self.y_len() {
            for x in 0..self.x_len() {
                let p = Position { x: x as i32, y: y as i32 };
                if seen.get(&p).is_some() {
                    print!("O");
                } else {
                    print!("{}", self.at(p).unwrap());
                }
            }
            println!();
        }
    }

    fn neighbors(&self, p: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if self.part2 {
            for d in vec![Direction::North, Direction::East, Direction::West, Direction::South].iter() {
                let position = p.go(*d);
                match self.at(position) {
                    None => continue,
                    Some(c) => match (c, d) {
                        ('#', _) => continue,
                        _ => neighbors.push(position),
                    },
                }
            }
        }
        else {
            match self.at(p).unwrap() {
                '>' => return vec![p.right()],
                '<' => return vec![p.left()],
                '^' => return vec![p.up()],
                'v' => return vec![p.down()],
                _ => {}
            }

            for d in vec![Direction::North, Direction::East, Direction::West, Direction::South].iter() {
                let position = p.go(*d);
                match self.at(position) {
                    None => continue,
                    Some(c) => match (c, d) {
                        ('#', _) => continue,
                        ('>', _) => if *d == Direction::East { neighbors.push(position); },
                        ('<', _) => if *d == Direction::West { neighbors.push(position); },
                        ('^', _) => if *d == Direction::North { neighbors.push(position); },
                        ('v', _) => if *d == Direction::South { neighbors.push(position); },
                        _ => neighbors.push(position),
                    },
                }
            }
        }

        neighbors
    }

    fn dfs(&self, p: Position, seen: &mut HashSet<Position>, steps: usize, ends: &mut Vec<usize>) {
        // If we reach the end, we'll want to keep track of it.

        if p == self.end() {
            ends.push(steps);
            return;
        }

        // If we've already seen this point, we don't need to continue.
        if seen.contains(&p) {
            return;
        }

        // Add myself to the seen set and go down all my neighbors.
        seen.insert(p);

        // println!();
        // self.print(seen);

        let neighbors = self.neighbors(p);
        for n in neighbors {
            if seen.contains(&n) {
                continue;
            }
            self.dfs(n, seen, steps + 1, ends);
        }
        // remove myself for the other branches.
        seen.remove(&p);
    }

    fn longest_path_dfs(&self) -> usize {
        let mut ends = Vec::new();
        let mut seen = HashSet::new();
        self.dfs(self.start(), &mut seen, 0, &mut ends);

        *ends.iter().max().unwrap()
    }
}


fn process(input: &str, part2: bool) -> usize {
    let g = Grid::new(input, part2);

    g.longest_path_dfs()
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn part1() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(94, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(154, process(input, true));
    }
}
