# Rust Advent of Code CLI and folder structure
My CLI tool for generating Advent of Code puzzles in Rust.

After cloning, start by typing:
```sh
cargo run <year> <day>
```
This will generate the code for that year and day, executing that same command again runs the two parts.
```rs
// src/puzzles/y24/d1.rs

fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> u32 {
    0
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
        let input = "";
        let expected = 0;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2024 Day 1 Part 2
    fn y24d1p2() {
        let input = "";
        let expected = 0;
        assert_eq!(part2(input), expected);
    }
}
```
Next you will have to paste your input into the created text file at inputs/y24/d1.txt

## Testing
While solving the puzzle you will want to test the code.
Run all tests
```sh
cargo test
```
Run all tests in year 24
```sh
cargo test y24
```
Run all tests of year 24 day 1
```sh
cargo test y24d1
```
Run part 1 of year 24 day 1
```sh
cargo test y24d1p1
```

## Folder structure
```
advent-of-code/
├── src/
│   ├── main.rs
│   ├── utils.rs
│   ├── puzzles/
│   │   ├── mod.rs
│   │   ├── y22/
│   │   │   ├── mod.rs
│   │   │   ├── d1.rs
├── inputs/
│   │   ├── y22/
│   │   │   ├── d1.txt
```
