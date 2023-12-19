#![allow(warnings, unused)]

use std::collections::{HashMap, HashSet, VecDeque};
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
    Done, // Indicates last rule
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Rule {
    field: Field,
    threshold: Threshold,
    value: u32,
    destination: String,
}

impl Rule {
    fn new(s: &str) -> Self {
        if !s.contains(':') {
            Rule {
                field: Field::X,
                value: 0,
                threshold: Threshold::Done,
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
                threshold: rule,
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

// Noticed they are in order: xmas
fn parse_part(s: &str) -> Vec<u32> {
    s.split(",").map(|w| {
        let (_, x) = w.split_once("=").unwrap();
        return x.parse().unwrap();
    }).collect_vec()
}

fn travel(set: &HashMap<String, Vec<Rule>>, part: &Vec<u32>, destination: &str) -> bool {
    if destination == "A" {
        return true;
    }
    if destination == "R" {
        return false;
    }

    let rules = set.get(destination).unwrap();

    for rule in rules {
        let value = part[rule.field as usize];

        if rule.threshold == Threshold::Done {
            return travel(set, part, rule.destination.as_str());
        }

        if rule.threshold == Threshold::Greater && value > rule.value {
            return travel(set, part, rule.destination.as_str());
        }

        if rule.threshold == Threshold::Lesser && value < rule.value {
            return travel(set, part, rule.destination.as_str());
        }
    }

    unreachable!();
}

fn start_travel(set: &HashMap<String, Vec<Rule>>, part: &Vec<u32>) -> bool {
    let start = set.get("in").unwrap();
    return travel(set, part, "in");
}

fn process(input: &str, part2: bool) -> u32 {
    let (w, p) = input.split_once("\n\n").unwrap();

    let parts = p.lines().map(|l| {
        let result = &l[1..l.len() - 1];
        return parse_part(result);
    }).collect_vec();


    let rule_set = w.lines().map(|l| {
        let div = l.find('{').unwrap();
        let name = l[..div].to_string();
        let rules = l[div + 1..l.len() - 1].split(',').map(|r| Rule::new(r)).collect_vec();
        (name, rules)
    }).collect::<HashMap<String, Vec<Rule>>>();


    return parts.iter().filter_map(|part| {
        return if start_travel(&rule_set, part) {
            Some(part.iter().sum::<u32>())
        } else {
            None
        }
    }).sum::<u32>();
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
        assert_eq!(19114, process(input, false));
    }

    #[test]
    fn part2() {
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
        assert_eq!(19114, process(input, false));
    }
}
