fn part1(input: &str) -> u32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let valid = check(numbers.clone());
        if valid {
            safe_count += 1;
            continue;
        }
    }
    safe_count
}

fn check(numbers: Vec<u32>) -> bool {
    let mut nums_iter = numbers.iter();
    let mut is_ascending: Option<bool> = None;
    let mut valid = true;
    let mut prev_num = nums_iter.next().unwrap();
    for num in nums_iter {
        let abs_diff = prev_num.abs_diff(*num);
        if !(1..=3).contains(&abs_diff) {
            valid = false;
        }
        if is_ascending.is_none() {
            if prev_num < num {
                is_ascending = Some(true);
            } else {
                is_ascending = Some(false);
            }
        }
        if is_ascending.is_some_and(|x| x) && prev_num > num {
            valid = false;
        }
        if is_ascending.is_some_and(|x| !x) && prev_num < num {
            valid = false;
        }
        prev_num = num;
    }
    valid
}

fn part2(input: &str) -> u32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let valid = check(numbers.clone());
        if valid {
            safe_count += 1;
            continue;
        }

        let mut group_result = vec![];

        for i in 0..numbers.len() {
            let mut mutated_numbers = numbers.clone();
            mutated_numbers.remove(i);
            group_result.push(check(mutated_numbers));
        }

        if group_result.iter().any(|x| *x) {
            safe_count += 1;
        }
    }
    safe_count
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y24/d2.txt").expect("Failed to read input file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Year 2024 Day 2 Part 1
    fn y24d2p1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let expected = 2;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 2 Part 2
    fn y24d2p2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let expected = 4;
        assert_eq!(part2(input), expected);
    }

    #[test]
    fn first_p2() {
        let input = "1 5 6 7 9";
        let expected = 1;
        assert_eq!(part2(input), expected);
    }

    #[test]
    fn last_p2() {
        let input = "1 3 6 7 20";
        let expected = 1;
        assert_eq!(part2(input), expected);
    }

    #[test]
    fn snippet1_p2() {
        let input = "1 20 21 22
20 50 21 22
20 21 22 50
20 21 50 22";
        let expected = 4;
        assert_eq!(part2(input), expected);
    }
}
