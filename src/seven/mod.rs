#![allow(warnings, unused)]

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::intrinsics::drop_in_place;
use itertools::Itertools;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Hand {
    hand: HandType,
    cards: [u8; 5],
    bid: i64,
}

impl Hand {
    pub fn new(s: &str, part2: bool) -> Hand {
        let (h, b) = s.split_once(" ").unwrap();

        let hand = TryInto::<[u8; 5]>::try_into(h.as_bytes()).unwrap().map(|c| match c {
            b'2'..=b'9' => c - b'0',
            b'T' => 10,
            b'J' => if part2 { 0 } else { 11 },
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => unreachable!(),
        });

        let mut joker_count = 0;

        let mut frequency = hand
            .iter()
            .sorted_unstable()
            .group_by(|&&c| c)
            .into_iter()
            .filter_map(|(c, g)| {
                if part2 && c == 0 {
                    joker_count = g.count();
                    None
                } else {
                    Some(g.count())
                }
            })
            .sorted()
            .collect_vec();

        match frequency.last_mut() {
            Some(f) => *f += joker_count,
            None => frequency.push(joker_count),
        }

        let hand_type = match frequency.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Hand {
            hand: hand_type,
            cards: hand,
            bid: b.parse::<i64>().unwrap(),
        }
    }

    pub fn cmp(&self, other: &Hand) -> Ordering {
        (&self).bid.cmp(&other.bid)
    }
}

fn process(input: &str, part2: bool) -> i64 {
    let mut hands = input.lines().map(|l| {
        return Hand::new(l,part2);
    }).sorted_unstable().collect::<Vec<Hand>>();

    return hands.into_iter().enumerate().map(|(i, h)| {
        return (i as i64 + 1) * h.bid;
    }).sum::<i64>();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905, process(input, true));
    }


}
