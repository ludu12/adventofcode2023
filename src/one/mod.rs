pub fn run() {
    let input = include_str!("input.txt");
    let output = process(input);
    println!("Output: {}", output.to_string());
}

fn process(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let mut it =
                line.chars().filter_map(|character| {
                    character.to_digit(10)
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
    fn run() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input).to_string());
    }
}
