use std::char;

use aoc_runner_derive::{aoc, aoc_lib};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, e| {
        let numbers = e.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
        acc + format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap()
    })
}

aoc_lib! { year = 2023 }
