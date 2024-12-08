use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    let mut direction_xy: (i32, i32) = (0, -1);
    let mut guard_position: (i32, i32) = (0, 0);
    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert((x as i32, y as i32));
            } else if c == '^' {
                guard_position = (x as i32, y as i32);
            }
        }
    }
    let mut guard_visited: HashSet<(i32, i32)> = HashSet::new();
    while guard_position.0 >= 0
        && guard_position.0 <= (input.lines().next().unwrap().len() - 1) as i32
        && guard_position.1 >= 0
        && guard_position.1 <= (input.lines().count() - 1) as i32
    {
        if walls.contains(&(
            guard_position.0 + direction_xy.0,
            guard_position.1 + direction_xy.1,
        )) {
            direction_xy = match direction_xy {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                _ => (0, -1),
            }
        }
        guard_visited.insert(guard_position);
        guard_position.0 += direction_xy.0;
        guard_position.1 += direction_xy.1;
    }

    guard_visited.len() as u32
}

fn part2(input: &str) -> u32 {
    0
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y24/d6.txt").expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y24d6right_bound() {
        let input = "....#.....
..........
..........
....^.....";
        let expected = 8;
        assert_eq!(part1(input), expected);
    }
    #[test]
    fn y24d6circle() {
        let input = "....#.....
.........#
#.........
....^...#.";
        let expected = 16;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 6 Part 1
    fn y24d6p1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let expected = 41;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 6 Part 2
    fn y24d6p2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let expected = 0;
        assert_eq!(part2(input), expected);
    }
}
