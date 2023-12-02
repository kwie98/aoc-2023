use std::fs;

fn main() {
    let day = DAY;
    let input =
        fs::read_to_string(format!("data/day{:02}.txt", day)).expect("Could not read input");
    println!("Day {}, part 1: {}", day, part_1(&input));
    println!("Day {}, part 2: {}", day, part_2(&input));
}

fn part_1(input: &str) -> &str {
    input
}

fn part_2(input: &str) -> &str {
    input
}
