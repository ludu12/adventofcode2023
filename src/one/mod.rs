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
            let mut it = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let mut result = reduced_line.chars().next().unwrap();

                if part2 {
                    if reduced_line.starts_with("one") {
                        result = '1';
                    } else if reduced_line.starts_with("two") {
                        result = '2';
                    } else if reduced_line.starts_with("three") {
                        result = '3';
                    } else if reduced_line.starts_with("four") {
                        result = '4';
                    } else if reduced_line.starts_with("five") {
                        result = '5';
                    } else if reduced_line.starts_with("six") {
                        result = '6';
                    } else if reduced_line.starts_with("seven") {
                        result = '7';
                    } else if reduced_line.starts_with("eight") {
                        result = '8';
                    } else if reduced_line.starts_with("nine") {
                        result = '9';
                    };
                }
                return result.to_digit(10);
            });

            let first = it.next().expect("should be a number");

            let num = match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }.parse::<u32>().expect("should be a number");

            return num;
        }).sum::<u32>();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input, false).to_string());
    }

    #[test]
    fn part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input, true).to_string());
    }
}
