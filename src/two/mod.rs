#![allow(warnings, unused)]

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> u32 {
    return input
        .lines()
        .map(|line| {
            let (i, d) = line.split_once(": ").unwrap();
            let id = i.split(" ").last()
                .expect("game should have an id")
                .parse::<u32>()
                .expect("game id should be a valid digit");

            let game = d.split("; ");

            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;

            for round in game {
                for cube in round.split(", ") {
                    let (n, color) = cube.split_once(" ").unwrap();
                    let num = n.parse::<u32>().expect("cube amount must be a number");

                    if color == "blue" {
                        blue = blue.max(num);
                    }
                    if color == "green" {
                        green = green.max(num);
                    }
                    if color == "red" {
                        red = red.max(num);
                    }
                }
            }

            if part2 {
                return blue * red * green;
            }
            else {
                if red <= 12 && green <= 13 && blue <= 14 {
                    return id;
                }
                return 0;
            }
        }).sum::<u32>();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "Game 11: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("18", process(input, false).to_string());
    }

    #[test]
    fn part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input, true).to_string());
    }
}
