#![allow(warnings, unused)]

use std::collections::HashMap;
use std::panic::panic_any;
use itertools::Itertools;
use crate::utils::print_grid;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Threshold {
    Greater,
    Lesser,
    Equal, // Indicates last rule
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Rule {
    field: Field,
    rule: Threshold,
    value: i64,
    destination: String,
}

impl Rule {
    fn new(s: &str) -> Self {
        if !s.contains(':') {
            Rule {
                field: Field::X,
                value: 0,
                rule: Threshold::Equal,
                destination: s.to_string(),
            }
        } else {
            let (c, d) = s.split_once(":").unwrap();

            let char = c.as_bytes()[0];
            let field = match char {
                b'x' => Field::X,
                b'm' => Field::M,
                b'a' => Field::A,
                b's' => Field::S,
                _ => unreachable!()
            };
            let compare = c.as_bytes()[1];
            let rule = match compare {
                b'>' => Threshold::Greater,
                b'<' => Threshold::Lesser,
                _ => unreachable!()
            };

            let value = (&c[2..c.len()]).parse().unwrap();
            Rule {
                destination: d.to_string(),
                value,
                rule,
                field,
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Field {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn parse_field(s: &str) -> i64 {
        let (_, x) = s.split_once("=").unwrap();
        return x.parse().unwrap();
    }

    fn new(s: &str) -> Self {
        let mut it = s.split(",");
        let x = Part::parse_field(it.next().unwrap());
        let m = Part::parse_field(it.next().unwrap());
        let a = Part::parse_field(it.next().unwrap());
        let s = Part::parse_field(it.next().unwrap());
        Part {
            x,
            m,
            a,
            s,
        }
    }

    fn get_field(&self, field: Field) -> i64 {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s
        }
    }
}


fn process(input: &str, part2: bool) -> i64 {
    let (w, p) = input.split_once("\n\n").unwrap();

    let parts = p.lines().map(|l| {
        let result = &l[1..l.len() - 1];
        return Part::new(result);
    }).collect_vec();

    let rules = w.lines().map(|l| {
        let div = l.find('{').unwrap();
        let name = &l[..div];
        let rules = l[div + 1..l.len() - 1].split(',').map(|r| Rule::new(r)).collect_vec();
        (name, rules)
    }).collect::<HashMap<&str, Vec<Rule>>>();

    dbg!(rules);
    return 1;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(1, process(input, false));
    }
}
