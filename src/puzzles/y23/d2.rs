fn part1(input: &str) -> u32 {
    let max_rgb = (12, 13, 14);

    let sum = input
        .lines()
        .map(|line| -> u32 {
            let line = line.replace("Game", "").replace(" ", "");
            let id_colors: Vec<&str> = line.split(':').collect();
            let id: u32 = id_colors[0].parse().unwrap();
            let mut possible = true;
            for set in id_colors[1].split(';') {
                for count_cube in set.split(',') {
                    let mut num = String::from("");
                    let mut color = String::from("");
                    for char in count_cube.chars() {
                        if char.is_digit(10) {
                            num.push(char);
                        } else {
                            color.push(char);
                        }
                    }
                    let numnum: u32 = num.parse::<u32>().unwrap();
                    if color == "red" && numnum > max_rgb.0 {
                        possible = false;
                    } else if color == "green" && numnum > max_rgb.1 {
                        possible = false;
                    } else if color == "blue" && numnum > max_rgb.2 {
                        possible = false;
                    }
                }
            }
            if possible {
                return id;
            }
            0
        })
        .sum();

    sum
}

fn part2(input: &str) -> u32 {
    let sum = input
        .lines()
        .map(|line| -> u32 {
            let line = line.replace("Game", "").replace(" ", "");
            let id_colors: Vec<&str> = line.split(':').collect();
            let mut maxrgb = (1, 1, 1);
            for set in id_colors[1].split(';') {
                for count_cube in set.split(',') {
                    let mut num = String::from("");
                    let mut color = String::from("");
                    for char in count_cube.chars() {
                        if char.is_digit(10) {
                            num.push(char);
                        } else {
                            color.push(char);
                        }
                    }
                    let numnum: u32 = num.parse::<u32>().unwrap();
                    if color == "red" && numnum > maxrgb.0 {
                        maxrgb.0 = numnum;
                    } else if color == "green" && numnum > maxrgb.1 {
                        maxrgb.1 = numnum;
                    } else if color == "blue" && numnum > maxrgb.2 {
                        maxrgb.2 = numnum;
                    }
                }
            }
            maxrgb.0 * maxrgb.1 * maxrgb.2
        })
        .sum();

    sum
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/y23/d2.txt").expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Year 2023 Day 2 Part 1
    fn y23d2p1() {
        let input = "";
        let expected = 0;
        assert_eq!(part1(input), expected);
    }

    #[test]
    // Year 2023 Day 2 Part 2
    fn y23d2p2() {
        let input = "";
        let expected = 0;
        assert_eq!(part2(input), expected);
    }
}
