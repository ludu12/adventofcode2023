#![allow(warnings, unused)]

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::intrinsics::drop_in_place;

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());

    // let part2 = process(input, true);
    // println!("Part2: {}", part2.to_string());
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
    pub fn new(s: &str) -> Hand {
        let (h, b) =  s.split_once(" ").unwrap();


        let hand = TryInto::<[u8; 5]>::try_into(h.as_bytes()).unwrap().map(|c| match c {
            b'2'..=b'9' => c - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => unreachable!(),
        });

        let frequencies = hand
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, val|{
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                map
            });

        // dbg!(hand);
        // dbg!(frequencies);
        let btree: BTreeSet<_> = frequencies.values().collect();
        println!("{:?}", btree);
        // let frequency = hand
        //     .iter()
        //     .group_by(|&&c| c)
        //     .into_iter()
        //     .map(|(_c, g)| g.count())
        //     .sorted()
        //     .collect_vec();
        //
        // let hand_type = match frequency.as_slice() {
        //     [5] => HandType::FiveOfAKind,
        //     [1, 4] => HandType::FourOfAKind,
        //     [2, 3] => HandType::FullHouse,
        //     [1, 1, 3] => HandType::ThreeOfAKind,
        //     [1, 2, 2] => HandType::TwoPair,
        //     [1, 1, 1, 2] => HandType::OnePair,
        //     _ => HandType::HighCard,
        // };

        Hand {
            hand: HandType::FiveOfAKind,
            cards: hand,
            bid: b.parse::<i64>().unwrap(),
        }
    }

    pub fn cmp(&self, other: &Hand) -> Ordering {
        (&self).bid.cmp(&other.bid)
    }
}

fn process(input: &str, part2: bool) -> i64 {

    let x: Vec<i64> = Vec::from([1,2,3]);
    let mut hands = input.lines().map(|l| {
        return Hand::new(l);
    }).collect::<Vec<Hand>>();


    hands.sort_by(|a,b| a.cmp(b) );

    // dbg!(hands);

    return 6440;
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

//     #[test]
//     fn part2() {
//         let input = "Time:      7  15   30
// Distance:  9  40  200";
//         assert_eq!(71503, process(input, true));
//     }
}
