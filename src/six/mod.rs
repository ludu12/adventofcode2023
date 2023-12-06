#![allow(warnings, unused)]

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input);
    println!("Part1: {}", part1.to_string());
}

fn process(input: &str) -> u32 {
    let (_, t) = input.lines().nth(0).unwrap().split_once(": ").unwrap();
    let (_, d) = input.lines().nth(1).unwrap().split_once(": ").unwrap();

    let times: Vec<u32> = t
        .split_whitespace()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = d
        .split_whitespace()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();


    let mut race_wins = vec![];

    for i in 0..times.len() {
        let race_time = times[i];
        let race_distance= distances[i];
        race_wins.push(0);

        for t in 0..race_time  {
            let t0 = t;
            let t1= race_time - t;

            let distance = t1 * t0;
            if distance > race_distance {
                race_wins[i] += 1;
            }
        }
    }


    let mut product: u32 = 1;
    for i in race_wins {
        product *= i;
    }

    return product;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input));
    }
}
