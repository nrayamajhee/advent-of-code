use crate::read_lines;
use anyhow::Result;

struct Line {
    password: String,
    letter: char,
    numbers: (u8, u8),
}

impl Line {
    pub fn parse(line: &str) -> Result<Self> {
        let split: Vec<&str> = line.split(":").collect();
        let password = split[1].trim().to_string();
        let split: Vec<&str> = split[0].split(" ").collect();
        let letter: char = split[1].parse()?;
        let split: Vec<&str> = split[0].split("-").collect();
        let numbers = (split[0].parse::<u8>()?, split[1].parse::<u8>()?);
        Ok(Self {
            password,
            letter,
            numbers,
        })
    }
}

pub fn part1(filename: &str) -> Result<usize> {
    let mut num_valid = 0;
    for each in read_lines(filename)? {
        let line = Line::parse(&each?)?;
        let num_letter = line.password.matches(line.letter).count() as u8;
        let (min, max) = line.numbers;
        if num_letter >= min && num_letter <= max {
            num_valid += 1;
        }
    }

    Ok(num_valid)
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut num_valid = 0;
    for each in read_lines(filename)? {
        let line = Line::parse(&each?)?;
        let chars: Vec<char> = line.password.chars().collect();
        let in_first = chars[(line.numbers.0 - 1) as usize] == line.letter;
        let in_second = chars[(line.numbers.1 - 1) as usize] == line.letter;
        if (in_first || in_second) && !(in_first && in_second) {
            num_valid += 1;
        }
    }

    Ok(num_valid)
}
