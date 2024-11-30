fn part1(input: &str) -> u32 {
    let instuct_string = input.split("\n\n").next().unwrap();

    let mapping_string = input.split("\n\n").last().unwrap();

    let map = mapping_string
        .lines()
        .map(|line| {
            let id = line.split(" = ").next().unwrap();
            let mut paths = line.split(" = ").last().unwrap().split(", ");
            let first = paths.next().unwrap();
            let last = paths.next().unwrap();
            (id, (&first[1..], &last[..last.len() - 1]))
        })
        .collect::<BTreeMap<&str, (&str, &str)>>();

    // for i in map.into_iter() {
    //     println!("{} = ({}, {})", i.0, i.1 .0, i.1 .1);
    // }

    let mut count: u32 = 0;
    let mut instruction_cursor: usize = 0;
    let mut cursor = "AAA";

    while cursor != "ZZZ" {
        for c in instuct_string.chars() {
            let paths = map.get(cursor).expect("Should be a key");
            if c == 'L' {
                cursor = paths.0;
            } else {
                cursor = paths.1;
            }

            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> u32 {
    0
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y23/d8.txt").expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Year 2023 Day 8 Part 1
    fn y23d8p1() {
        let input = "";
        let expected = 0;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2023 Day 8 Part 2
    fn y23d8p2() {
        let input = "";
        let expected = 0;
        assert_eq!(part2(input), expected);
    }
}
