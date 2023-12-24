#![allow(warnings, unused)]

use std::collections::HashSet;
use std::fmt;
use std::ops::Range;
use itertools::Itertools;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, 200000000000000.0..400000000000000.0, false);
    println!("Part1: {}", part1.to_string());
    // let part2 = process(input, 200000000000000.0..400000000000000.0, true);
    // println!("Part2: {}", part2.to_string());
}

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    x: f32,
    y: f32,
    // z: i32,
    vx: f32,
    vy: f32,
    // vz: i32,

    a: f32,
    b: f32,
    c: f32,
}

impl fmt::Display for Hailstone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({{{},{}}} a={}, b={}, c={})", self.x, self.y, self.a, self.b, self.c)
    }
}

impl Hailstone {
    fn new(line: &str) -> Hailstone {
        let (pos, vel) = line.split_once(" @ ").unwrap();

        let mut pos_split = pos.split(", ");
        let x = pos_split.next().unwrap().trim().parse::<f32>().unwrap();
        let y = pos_split.next().unwrap().trim().parse::<f32>().unwrap();
        // let z = pos_split.next().unwrap().trim().parse().unwrap();

        let mut vel_split = vel.split(", ");
        let vx = vel_split.next().unwrap().trim().parse::<f32>().unwrap();
        let vy = vel_split.next().unwrap().trim().parse::<f32>().unwrap();
        // let vz = vel_split.next().unwrap().trim().parse().unwrap();

        /*
           ax + by = c --- (x,y) + t(dx,dy) = (x1,y1)

           px = x - vx*t --- t = (px - x) / vx
           py = y - vy*t --- t = (py - y) / vy

           (px - x) / vx = (py - y) / vy
           vy(px - x) = vx(py - y)
           vy*px - vy*x = vx*py - vx*y

             ax  +  by   =  c
           vy*px - vx*py = -vx*y + vy*x
        */

        let a = vy;
        let b = -vx;
        let c = -vx * y + vy * x;

        return Hailstone { x, y, vx, vy, a, b, c };
    }

    fn intersects_xy(&self, other: &Hailstone, bounds: &Range<f32>) -> bool {
        let a1 = self.a;
        let a2 = other.a;
        let b1 = self.b;
        let b2 = other.b;
        let c1 = self.c;
        let c2 = other.c;

        // check parallel a1/b1 = a2/b2 --- a1 * b2 = a2 * b1
        if a1 * b2 == a2 * b1 {
            return false;
        }

        // a1*x + b1*y = c1
        // a2*x + b2*y = c2

        // Solve for x
        // a1*b2*x + b1*b2*y = c1*b2
        // a2*b1*x + b2*b1*y = c2*b1
        // a1*b2*x - a2*b1*x = c1*b2 - c2*b1
        let div = (a1 * b2 - a2 * b1); // this is the same for both
        let x = (c1 * b2 - c2 * b1) / div;

        // Solve for y
        // a2*a1*x + b2*a1*y = c2*a1
        // a1*a2*x + b1*a2*y = c1*a2
        // b2*a1*y - b1*a2*y = c2*a1 - c1*a2
        let y = (c2 * a1 - c1 * a2) / div;


        // make sure they are not in the past
        if ((x - self.x) * self.vx) >= 0f32
            && ((y - self.y) * self.vy) >= 0f32
            && ((x - other.x) * other.vx) >= 0f32
            && ((x - other.x) * other.vx) >= 0f32 {

            // Make sure within bounds
            if bounds.contains(&x) && bounds.contains(&y) {
                return true;
            }
        }

        return false;
    }
}

fn process(input: &str, bounds: Range<f32>, part2: bool) -> usize {
    let hailstones = input.lines().map(|l| Hailstone::new(l)).collect_vec();

    let mut total = 0;
    for i in 0..hailstones.len() {
        let h1 = hailstones[i];
        for j in ((i + 1)..hailstones.len()) {
            let h2 = hailstones[j];

            if (h1.intersects_xy(&h2, &bounds)) {
                total += 1;
            }
        }
    }

    return total;
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn part1() {
        let input = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(2, process(input, 7.0..27.0, false));
    }
}
