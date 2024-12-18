use std::time::Instant;

fn check_horizontal(line: &str) -> u32 {
    line.as_bytes()
        .windows(4)
        .filter(|&w| w == "XMAS".as_bytes() || w == "SAMX".as_bytes())
        .count() as u32
}

fn check_char(line: &str, index: usize, search: &str, search_index: usize) -> bool {
    line.as_bytes()
        .get(index)
        .is_some_and(|&c| c == search.as_bytes()[search_index])
}

fn check_x_mas(
    index: usize,
    reversed: bool,
    current_line: &str,
    next_line: &str,
    next_next_line: &str,
) -> u32 {
    let search = if reversed { "SAM" } else { "MAS" };
    if current_line.len() - index > 2
        && check_char(next_line, index + 1, search, 1)
        && check_char(current_line, index + 2, search, 0)
        && check_char(next_next_line, index + 2, search, 2)
        && check_char(next_next_line, index, search, 2)
    {
        return 1;
    }
    if current_line.len() - index > 2
        && check_char(next_line, index + 1, search, 1)
        && check_char(current_line, index + 2, search, 2)
        && check_char(next_next_line, index + 2, search, 2)
        && check_char(next_next_line, index, search, 0)
    {
        return 1;
    }

    0
}
fn check_xmas(
    index: usize,
    reversed: bool,
    next_line: &str,
    next_next_line: &str,
    next_next_next_line: &str,
) -> u32 {
    let mut correct = 0;
    let search = if reversed { "SAMX" } else { "XMAS" };
    if check_char(next_line, index, search, 1)
        && check_char(next_next_line, index, search, 2)
        && check_char(next_next_next_line, index, search, 3)
    {
        // check down
        correct += 1;
    }
    if next_line.len() - index > 3
        && check_char(next_line, index + 1, search, 1)
        && check_char(next_next_line, index + 2, search, 2)
        && check_char(next_next_next_line, index + 3, search, 3)
    {
        // check bottom right
        correct += 1;
    }
    if index > 2
        && check_char(next_line, index - 1, search, 1)
        && check_char(next_next_line, index - 2, search, 2)
        && check_char(next_next_next_line, index - 3, search, 3)
    {
        // check bottom left
        correct += 1
    }
    correct
}

fn part1(input: &str) -> u32 {
    let mut appearances = 0;
    let mut third_previous_line: Option<&str> = None;
    let mut second_previous_line: Option<&str> = None;
    let mut first_previous_line: Option<&str> = None;
    for line in input.lines() {
        appearances += check_horizontal(line);
        if let Some(third) = third_previous_line {
            for (index, character) in third.chars().enumerate() {
                if character == 'S' || character == 'X' {
                    let reversed = character == 'S';
                    let result = check_xmas(
                        index,
                        reversed,
                        second_previous_line.unwrap(),
                        first_previous_line.unwrap(),
                        line,
                    );
                    appearances += result;
                }
            }
        }
        third_previous_line = second_previous_line;
        second_previous_line = first_previous_line;
        first_previous_line = Some(line);
    }
    appearances
}

fn part2(input: &str) -> u32 {
    let mut appearances = 0;
    let mut second_previous_line: Option<&str> = None;
    let mut first_previous_line: Option<&str> = None;
    for line in input.lines() {
        if let Some(second) = second_previous_line {
            for (index, character) in second.chars().enumerate() {
                if character == 'S' || character == 'M' {
                    let is_second = character == 'S';
                    let result = check_x_mas(
                        index,
                        is_second,
                        second_previous_line.unwrap(),
                        first_previous_line.unwrap(),
                        line,
                    );
                    appearances += result;
                }
            }
        }
        second_previous_line = first_previous_line;
        first_previous_line = Some(line);
    }
    appearances
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y24/d4.txt").expect("Failed to read input file");
    let part1_start = Instant::now();
    let part1_result = part1(&input);
    let part1_duration = part1_start.elapsed().as_micros();
    let part2_start = Instant::now();
    let part2_result = part2(&input);
    let part2_duration = part2_start.elapsed().as_micros();
    println!("Part 1: {} in {}μs", part1_result, part1_duration);
    println!("Part 2: {} in {}μs", part2_result, part2_duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Year 2024 Day 4 Part 1
    fn y24d4p1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let expected = 18;
        assert_eq!(part1(input), expected);
    }
    #[test]
    fn y24d4p1right() {
        let input = "XMAS";
        let expected = 1;
        assert_eq!(part1(input), expected);
    }
    #[test]
    fn y24d4p1rightreversed() {
        let input = "SAMX";
        let expected = 1;
        assert_eq!(part1(input), expected);
    }
    #[test]
    fn y24d4p1bottomright() {
        let input = "X...
.M..
..A.
...S";
        let expected = 1;
        assert_eq!(part1(input), expected);
    }
    #[test]
    fn y24d4p1bottomleft() {
        let input = "...X
..M.
.A..
S...";
        let expected = 1;
        assert_eq!(part1(input), expected);
    }
    #[test]
    fn y24d4p1down() {
        let input = "X
M
A
S";
        let expected = 1;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 4 Part 2
    fn y24d4p2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let expected = 9;
        assert_eq!(part2(input), expected);
    }
}
