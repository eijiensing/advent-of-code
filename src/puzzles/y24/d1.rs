use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let mut left_vec: Vec<i32> = vec![];
    let mut right_vec: Vec<i32> = vec![];
    for line in input.lines() {
        let mut splitted = line.split("   ");
        let left = splitted.next().unwrap().parse::<i32>().unwrap();
        let right = splitted.next().unwrap().parse::<i32>().unwrap();
        left_vec.push(left);
        right_vec.push(right);
    }
    let sum_left: i32 = left_vec.iter().sum();
    let sum_right: i32 = right_vec.iter().sum();
    let xd = sum_left - sum_right;
    xd.unsigned_abs()
}

fn part2(input: &str) -> u32 {
    let mut score = 0;
    let mut left_vec = Vec::new();
    let mut right_map = HashMap::new();
    for line in input.lines() {
        // O(N)
        let mut splitted = line.split("   ");
        let left = splitted.next().unwrap().parse::<i32>().unwrap();
        let right = splitted.next().unwrap().parse::<i32>().unwrap();
        left_vec.push(left);
        *right_map.entry(right).or_insert(0) += 1;
    }
    for i in left_vec {
        if let Some(value) = right_map.get(&i) {
            score += i * value;
        }
    }
    score as u32
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y24/d1.txt").expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Year 2024 Day 1 Part 1
    fn y24d1p1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let expected = 11;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 1 Part 2
    fn y24d1p2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let expected = 31;
        assert_eq!(part2(input), expected);
    }
}
