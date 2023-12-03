use std::{collections::HashMap, fs};

fn main() {
    let day = 1;
    let input = fs::read_to_string(format!("data/day{:02}.txt", day)).unwrap();
    println!("Day {}, part 1: {}", day, part_1(&input));
    println!("Day {}, part 2: {}", day, part_2(&input));
}

fn part_1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let line = line;
            let digits: Vec<u32> = line.chars().filter_map(|c| char::to_digit(c, 10)).collect();
            assert!(digits.len() > 0, "Could not find any digits in {}", line);
            return digits[0] * 10 + digits[digits.len() - 1];
        })
        .sum();
}

fn part_2(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let line = line;
            let digit_names = HashMap::from([
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]);

            let digits: Vec<u32> = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    // First, check if we have a simple digit:
                    if c.is_ascii_digit() {
                        return char::to_digit(c, 10);
                    }
                    // Otherwise, check if a digit name starts at this position:
                    for &name in digit_names.keys() {
                        let name_starts_here =
                            line.get(i..i + name.len()).is_some_and(|sub| sub == name);
                        if name_starts_here {
                            return digit_names.get(name).copied();
                        }
                    }
                    return None;
                })
                .collect();
            assert!(digits.len() > 0, "Could not find any digits in {}", line);
            return digits[0] * 10 + digits[digits.len() - 1];
        })
        .sum();
}
