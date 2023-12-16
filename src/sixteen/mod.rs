#![allow(warnings, unused)]

use std::cmp::min;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::format;
use std::os::unix::raw::{gid_t, off_t};
use std::panic::panic_any;
use itertools::{fold, Itertools, join};
use crate::utils::{grid, print_grid, transpose};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum Direction {
    South,
    North,
    East,
    West,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Beam {
    direction: Direction,
    point: Point,
}

impl Beam {
    fn new(point: Point, direction: Direction) -> Self {
        Beam {
            point,
            direction,
        }
    }

    fn out_of_bounds(&self, grid: &Vec<Vec<char>>) -> bool {
        self.point.x < 0
            || self.point.y < 0
            || self.point.x >= grid[0].len() as i32
            || self.point.y >= grid.len() as i32
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.point.y -= 1,
            Direction::South => self.point.y += 1,
            Direction::West => self.point.x -= 1,
            Direction::East => self.point.x += 1,
        }
    }
}

fn travel(grid: &Vec<Vec<char>>, seen: &mut HashSet<Beam>, beam: Beam) {
    let mut new_beam = beam.clone();

    while !new_beam.out_of_bounds(&grid) && !seen.contains(&new_beam) {
        seen.insert(new_beam.clone());

        let c = grid[new_beam.point.y as usize][new_beam.point.x as usize];

        match (c, &new_beam.direction, ) {

            // Make a new one and travel
            ('|', Direction::East | Direction::West) => {
                let mut north = Beam::new(new_beam.point.clone(), Direction::North);
                travel(grid, seen, north);

                new_beam.direction = Direction::South;
            }

            // Make a new one and travel
            ('-', Direction::North | Direction::South) => {
                let mut east = Beam::new(new_beam.point.clone(), Direction::East);
                travel(grid, seen, east);

                new_beam.direction = Direction::West;
            }

            ('/', Direction::North) => new_beam.direction = Direction::East,
            ('/', Direction::South) => new_beam.direction = Direction::West,
            ('/', Direction::West) => new_beam.direction = Direction::South,
            ('/', Direction::East) => new_beam.direction = Direction::North,
            ('\\', Direction::North) => new_beam.direction = Direction::West,
            ('\\', Direction::South) => new_beam.direction = Direction::East,
            ('\\', Direction::West) => new_beam.direction = Direction::North,
            ('\\', Direction::East) => new_beam.direction = Direction::South,
            _ => {}
        }

        new_beam.move_forward();
    }
}

fn start_travel(grid: &Vec<Vec<char>>, beam: Beam) -> HashSet<Point> {
    let mut seen = HashSet::new();
    travel(grid, &mut seen, beam);

    let mut energized = HashSet::new();

    for b in seen.clone() {
        energized.insert(b.point);
    }

    return energized;
}

fn process(input: &str, part2: bool) -> usize {
    let g = grid(input);


    if part2 {
        let max_y = g.len();
        let max_x = g[0].len();

        let mut max = 0;

        for i in 0..max_x {
            let going_south = start_travel(&g, Beam::new(Point::new(i as i32, 0), Direction::South)).len();
            max = max.max(going_south);
            let going_north = start_travel(&g, Beam::new(Point::new(i as i32, (max_y - 1) as i32), Direction::North)).len();
            max = max.max(going_north);
        }

        for i in 0..max_y {
            let going_east = start_travel(&g, Beam::new(Point::new(0, i as i32), Direction::East)).len();
            max = max.max(going_east);
            let going_west = start_travel(&g, Beam::new(Point::new((max_x - 1) as i32, i as i32), Direction::West)).len();
            max = max.max(going_west);
        }

        return max;
    } else {
        return start_travel(&g, Beam::new(Point::new(0, 0), Direction::East)).len();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(46, process(input, false));
    }

    #[test]
    fn part2() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(51, process(input, true));
    }
}
