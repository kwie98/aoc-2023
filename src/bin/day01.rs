use std::{
    collections::{HashMap, LinkedList},
    fs,
};

use once_cell::sync::Lazy;

static DIGITS: Lazy<HashMap<&str, char>> = Lazy::new(|| {
    return HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
});

fn main() {
    let day = 1;
    let input = fs::read_to_string(format!("data/day{:02}.txt", day)).unwrap();
    println!("Day {}, part 1: {}", day, part_1(&input));
    println!("Day {}, part 2: {}", day, part_2(&input));
}

fn concat_first_last(digits: &mut dyn Iterator<Item = char>) -> Option<u32> {
    let first = digits.next();
    let last = digits.last(); // is None if there is only one digit in the line
    let mut first_last = String::with_capacity(2);
    match (first, last) {
        (Some(a), Some(b)) => {
            first_last.push(a);
            first_last.push(b);
        }
        // There might only be one digit. In that case, first and last are the same:
        (Some(a), None) => {
            first_last.push(a);
            first_last.push(a);
        }
        _ => {
            return None;
        }
    };
    return Some(
        first_last
            .parse()
            .unwrap_or_else(|_| panic!("Could not parse number {}", first_last)),
    );
}

fn parse_line_1(line: &str) -> u32 {
    let mut digits = line.chars().filter(|char| char.is_ascii_digit());
    return concat_first_last(&mut digits)
        .unwrap_or_else(|| panic!("Cold not parse line {}", line));
}

fn parse_line_2(line: &str) -> u32 {
    let mut digits: LinkedList<char> = LinkedList::new();
    let mut chars = line.chars();
    for i in 0..line.len() {
        // First, check if we have a simple digit:
        let current_char = chars.next().unwrap_or_else(|| panic!("Out of bounds"));
        if current_char.is_ascii_digit() {
            digits.push_back(current_char);
            continue;
        };

        // Otherwise, check if a digit name starts at this position:
        for &name in DIGITS.keys() {
            let name_starts_here = line
                .get(i..i + name.len())
                .map(|substr| substr == name)
                .is_some_and(|b| b);
            if name_starts_here {
                digits.push_back(*DIGITS.get(name).unwrap_or_else(|| panic!("ayo")))
            }
        }
    }
    return concat_first_last(&mut digits.into_iter())
        .unwrap_or_else(|| panic!("Could not parse line {}", line));
}

fn part_1(input: &str) -> u32 {
    return input.lines().map(|line| parse_line_1(line)).sum();
}

fn part_2(input: &str) -> u32 {
    return input.lines().map(|line| parse_line_2(line)).sum();
}
