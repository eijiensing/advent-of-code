fn part1(input: &str) -> u32 {
    0
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
    // Year 2024 Day 3 Part 1
    fn y24d3p1() {
        let input = "";
        let expected = 0;
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
