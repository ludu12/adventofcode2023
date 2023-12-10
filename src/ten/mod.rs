#![allow(warnings, unused)]

use std::panic::panic_any;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn go(grid: &Vec<Vec<char>>, i: usize, j: usize, direction: Direction) -> (usize, usize) {
    return match direction {
        Direction::North => {
            if i == 0 { (i, j) } else { (i - 1, j) }
        }
        Direction::East => {
            if j == grid[i].len() - 1 { (i, j) } else { (i, j + 1) }
        }
        Direction::South => {
            if i == grid.len() - 1 { (i, j) } else { (i + 1, j) }
        }
        Direction::West => {
            if j == 0 { (i, j) } else { (i, j - 1) }
        }
    };
}

fn get_direction(current: char, direction: Direction) -> Result<Direction, Direction> {
    return if current == '-' {
        match direction {
            Direction::East => Ok(Direction::East),
            Direction::West => Ok(Direction::West),
            _ => Err(direction)
        }
    } else if current == '|' {
        match direction {
            Direction::North => Ok(Direction::North),
            Direction::South => Ok(Direction::South),
            _ => Err(direction)
        }
    } else if current == 'L' {
        match direction {
            Direction::South => Ok(Direction::East),
            Direction::West => Ok(Direction::North),
            _ => Err(direction)
        }
    } else if current == 'J' {
        match direction {
            Direction::East => Ok(Direction::North),
            Direction::South => Ok(Direction::West),
            _ => Err(direction)
        }
    } else if current == '7' {
        match direction {
            Direction::East => Ok(Direction::South),
            Direction::North => Ok(Direction::West),
            _ => Err(direction)
        }
    } else if current == 'F' {
        match direction {
            Direction::North => Ok(Direction::East),
            Direction::West => Ok(Direction::South),
            _ => Err(direction)
        }
    }
    else if current == 'S' {
        Ok(direction)
    } else {
        Err(direction)
    };
}

fn traverse(grid: &Vec<Vec<char>>, i: usize, j: usize, direction: Direction) -> i32 {
    let mut next = grid[i][j];
    let mut next_i = i;
    let mut next_j = j;
    let mut next_direction = direction;
    let mut steps = 0;

    while next != 'S' {
        next_direction = get_direction(next, next_direction).expect(format!("Not of the loop! {}!", steps).as_str());
        (next_i, next_j) = go(grid, next_i, next_j, next_direction);
        next = grid[next_i][next_j];

        steps += 1;
    }

    return steps;
}

fn get_start_direction(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Direction {
    let n = go(grid, i, j, Direction::North);
    let north = grid[n.0][n.1];

    let e = go(grid, i, j, Direction::East);
    let east = grid[e.0][e.1];

    let s = go(grid, i, j, Direction::South);
    let south = grid[s.0][s.1];

    let w = go(grid, i, j, Direction::West);
    let west = grid[w.0][w.1];

    if ['|','F','7'].contains(&north) {
        return Direction::North;
    }

    if ['-','7','J'].contains(&east) {
        return Direction::East;
    }

    if ['|', 'J', 'L'].contains(&south) {
        return Direction::South;
    }

    if ['-', 'F', 'L'].contains(&west) {
        return Direction::West;
    }

    panic!("Could not find starting direction");
}

fn process(input: &str, part2: bool) -> i64 {
    let mut start_i: usize = 0;
    let mut start_j: usize = 0;

    let mut grid = vec![];

    input.lines().enumerate().for_each(|(i, l)| {
        let mut chars = vec![];

        l.chars().enumerate().for_each(|(j, c)| {
            if c == 'S' {
                start_i = i;
                start_j = j;
            }

            chars.push(c);
        });

        grid.push(chars);
    });

    let dir = get_start_direction(&grid,start_i, start_j);
    let (next_i, next_j) = go(&grid, start_i, start_j, dir);

    return (1 + traverse(&grid, next_i, next_j, dir) / 2) as i64;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1a() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, process(input, false));
    }

    #[test]
    fn part1b() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, process(input, false));
    }
}
