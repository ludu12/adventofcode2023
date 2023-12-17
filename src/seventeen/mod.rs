#![allow(warnings, unused)]

use std::cmp::{min, Ordering};
use std::cmp::max;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, format};
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

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "↑"),
            Direction::Right => write!(f, "→"),
            Direction::Down => write!(f, "↓"),
            Direction::Left => write!(f, "←"),
        }
    }
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn go(self, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point { x: self.x, y: self.y.wrapping_sub(1) },
            Direction::Right => Point { x: self.x + 1, y: self.y },
            Direction::Down => Point { x: self.x, y: self.y + 1 },
            Direction::Left => Point { x: self.x.wrapping_sub(1), y: self.y },
        }
    }

    fn is_in_bounds(self, max_x: usize, max_y: usize) -> bool {
        self.x <= max_x && self.y <= max_y
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct State {
    cost: i32,
    point: Point,
    momentum: Direction,
    steps: i32,
}

impl State {
    fn visited_key(&self) -> (Point, Direction, i32) {
        return (self.point, self.momentum, self.steps);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn find_path(grid: &Vec<Vec<u8>>, min_steps: i32, max_steps: i32) -> i32 {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let start = Point { x: 0, y: 0 };
    let target = Point { x: max_x, y: max_y };
    queue.push(State { cost: 0, point: start, momentum: Direction::Right, steps: 0 });
    queue.push(State { cost: 0, point: start, momentum: Direction::Down, steps: 0 });


    while let Some(q) = queue.pop() {
        if q.point == target && q.steps >= min_steps {
            // return the negative value as this is the true cost
            return -q.cost;
        }

        for direction in [Direction::Right, Direction::Down, Direction::Left, Direction::Up] {
            if direction == q.momentum.opposite()
                || (direction == q.momentum && q.steps >= max_steps)
                || (direction != q.momentum && q.steps < min_steps)
            {
                continue;
            }

            let new_point = match direction {
                Direction::Up => Point { x: q.point.x, y: q.point.y.wrapping_sub(1) },
                Direction::Right => Point { x: q.point.x + 1, y: q.point.y },
                Direction::Down => Point { x: q.point.x, y: q.point.y + 1 },
                Direction::Left => Point { x: q.point.x.wrapping_sub(1), y: q.point.y },
            };

            if !new_point.is_in_bounds(max_x, max_y) {
                continue;
            }

            // makes it a min heap
            let new_cost = q.cost - (grid[new_point.y][new_point.x] as i32);

            let next_state = State {
                cost: new_cost,
                point: new_point,
                momentum: direction,
                steps: if direction == q.momentum { q.steps + 1 } else { 1 },
            };

            // Add to heap if we haven't visited
            if visited.insert(next_state.visited_key()) {
                queue.push(next_state);
            }
        }
    }

    unreachable!();
}

fn process(input: &str, part2: bool) -> i32 {
    let g = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).expect("Invalid digit") as u8).collect_vec()).collect_vec();

    return if part2 {
        find_path(&g, 4, 10)
    } else {
        find_path(&g, 1, 3)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(102, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(94, process(input, true));
    }

    #[test]
    fn part2b() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(71, process(input, true));
    }
}
