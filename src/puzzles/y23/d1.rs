fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.split("\n") {
        let mut nums = Vec::<u32>::new();
        for character in line.chars() {
            if !character.is_digit(10) {
                continue;
            }
            nums.push(character.to_digit(10).unwrap())
        }
        sum += (nums.first().unwrap_or(&0) * 10) + nums.last().unwrap_or(&0);
    }

    return sum;
}

fn part2(input: &str) -> u32 {
    let string_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum: u32 = 0;
    for line in input.split("\n") {
        let mut nums = Vec::<u32>::new();
        for (index, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                nums.push(character.to_digit(10).unwrap())
            } else {
                for (index2, snums) in string_numbers.iter().enumerate() {
                    if line[index..].starts_with(snums) {
                        nums.push(index2 as u32 + 1);
                    }
                }
            }
        }
        sum += (nums.first().unwrap_or(&0) * 10) + nums.last().unwrap_or(&0);
    }

    sum
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y23/d1.txt").expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Year 2023 Day 1 Part 1
    fn y23d1p1() {
        let input = "";
        let expected = 0;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2023 Day 1 Part 2
    fn y23d1p2() {
        let input = "";
        let expected = 0;
        assert_eq!(part2(input), expected);
    }
}
