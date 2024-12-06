use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let mut correctly_ordered_updates = 0;
    let mut splitted = input.split("=\n");
    let rules = splitted.next().unwrap();
    let page_numbers = splitted.next().unwrap();
    // left is earlier than list on the right
    let mut left_earlier_than_right_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in rules.lines().map(|x| {
        let mut splitted2 = x.split("|");
        (
            splitted2.next().unwrap().parse::<u32>().unwrap(),
            splitted2.next().unwrap().parse::<u32>().unwrap(),
        )
    }) {
        left_earlier_than_right_map
            .entry(rule.0)
            .and_modify(|x| {
                x.push(rule.1);
            })
            .or_insert_with(|| vec![rule.1]);
    }

    for lines in page_numbers.lines() {
        let mut valid = true;
        let mut numbers = lines.split(",").map(|n| n.parse::<u32>().unwrap());
        let vec_nums = numbers.clone().collect::<Vec<u32>>();
        let middle_num = vec_nums.get(vec_nums.len() / 2).unwrap();
        let mut left_number = numbers.next().unwrap();
        for right_number in numbers {
            let left_number_list = left_earlier_than_right_map.get(&left_number);
            let right_number_list = left_earlier_than_right_map.get(&right_number);
            left_number = right_number;
            if left_number_list.is_some_and(|list| list.contains(&right_number)) {
                continue;
            }
            if right_number_list.is_some_and(|list| list.contains(&left_number)) {
                continue;
            }
            valid = false;
        }
        if valid {
            correctly_ordered_updates += middle_num;
        }
    }
    correctly_ordered_updates
}

fn part2(input: &str) -> u32 {
    0
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y24/d5.txt").expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Year 2024 Day 5 Part 1
    fn y24d5p1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
=
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let expected = 143;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 5 Part 2
    fn y24d5p2() {
        let input = "";
        let expected = 0;
        assert_eq!(part2(input), expected);
    }
}
