#![allow(warnings, unused)]

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    // let part2 = process(input, true);
    // println!("Part2: {}", part2.to_string());
}

fn create_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    for i in (0..lines.len()) {
        let mut row = Vec::new();

        for j in 0..lines[i].len() {
            row.push(lines[i].chars().nth(j).unwrap())
        }

        matrix.push(row);
    }

    return matrix;
}

fn check_for_symbol(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let row_min = if row == 0 { row } else { row - 1 };
    let row_max = if row == matrix.len() - 1 { row } else { row + 1 };

    let col_min = if col == 0 { col } else { col - 1 };
    // Assumption here is that each line is the same size
    let col_max = if col == matrix[0].len() - 1 { col } else { col + 1 };

    for i in (row_min..=row_max) {
        for j in (col_min..=col_max) {
            let elem = matrix[i][j];
            // Assumption here that all other values are symbols
            if elem != '.' && !elem.is_digit(10) {
                return true;
            }
        }
    }

    return false;
}

fn parse_to_num(str: &String) -> u32 {
    return match str.parse() {
        Ok(v) => v,
        Err(_) => 0
    };
}

fn process(input: &str, part2: bool) -> u32 {
    let matrix = create_matrix(input);
    let mut running_num: String = "".to_owned();
    let mut sum: u32 = 0;

    for (i, row) in matrix.iter().enumerate() {
        let mut running_num: String = "".to_owned();
        let mut has_symbol: bool = false;

        for (j, elem) in row.iter().enumerate() {
            if elem.is_digit(10) {
                running_num.push(*elem);
                has_symbol = if has_symbol { true } else { check_for_symbol(&matrix, i, j) }
            } else {
                if has_symbol {
                    sum += parse_to_num(&running_num);
                }
                running_num.clear();
                has_symbol = false;
            }
        }

        if has_symbol {
            sum += parse_to_num(&running_num);
        }
    }


    return sum;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "....977.+........................*........*610.....-....................................*.......489*795.......-....@545..915......*......641
...-.......123........@........113.....643............117.483../...................961.984..................878...........*........277..@...";

        assert_eq!("7867", process(input, false).to_string());
    }

//     #[test]
//     fn part2() {
//         let input = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";
//         assert_eq!("467835", process(input, true).to_string());
//     }
}
