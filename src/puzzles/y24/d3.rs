fn multiply_line(line: &str) -> u32 {
    let mut chars = line.chars().peekable();
    let mut result = 0;
    while chars.peek().is_some() {
        let mut current = chars.clone();
        if current.by_ref().take(4).collect::<String>() != "mul(" {
            chars.next();
            continue;
        }
        chars.by_ref().take(4).for_each(drop);

        let mut current = chars.clone();
        let first_number: String = current.by_ref().take_while(|&c| c.is_numeric()).collect();
        if first_number.is_empty() {
            continue;
        }
        chars.by_ref().take(first_number.len()).for_each(drop);

        if chars.next() != Some(',') {
            continue;
        }

        let mut current = chars.clone();
        let second_number: String = current.by_ref().take_while(|&c| c.is_numeric()).collect();
        if second_number.is_empty() {
            continue;
        }
        chars.by_ref().take(second_number.len()).for_each(drop);

        if chars.next() != Some(')') {
            continue;
        }
        result += first_number.parse::<u32>().unwrap() * second_number.parse::<u32>().unwrap();
    }
    result
}

fn part1(input: &str) -> u32 {
    let mut results = 0;
    for line in input.lines() {
        results += multiply_line(line);
    }
    results
}

fn part2(input: &str) -> u32 {
    0
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y24/d3.txt").expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y24d3p1case1() {
        let input = "mul(46,44)";
        let expected = 2024;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn y24d3p1case2() {
        let input = "asdjbmul(sadiugb346,44)";
        let expected = 0;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 3 Part 1
    fn y24d3p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = 161;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 3 Part 2
    fn y24d3p2() {
        let input = "";
        let expected = 0;
        assert_eq!(part2(input), expected);
    }
}
