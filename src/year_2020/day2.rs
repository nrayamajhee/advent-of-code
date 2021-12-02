use crate::read_lines;
use anyhow::Result;

struct Line {
    password: String,
    letter: char,
    numbers: (u8, u8),
}

impl Line {
    pub fn parse(line: &str) -> Result<Self> {
        let split: Vec<_> = line.split([' ', '-', ':'].as_ref()).collect();
        let numbers = (split[0].parse::<u8>()?, split[1].parse::<u8>()?);
        let letter: char = split[2].parse()?;
        let password = split[4].to_string();
        Ok(Self {
            password,
            letter,
            numbers,
        })
    }
}

pub fn part1(filename: &str) -> Result<usize> {
    read_lines(filename)?.fold(Ok(0), |num_valid, each_line| {
        let line = Line::parse(&each_line?)?;
        let num_letters = line.password.matches(line.letter).count() as u8;
        let (min, max) = line.numbers;
        if num_letters >= min && num_letters <= max {
            Ok(num_valid? + 1)
        } else {
            num_valid
        }
    })
}

pub fn part2(filename: &str) -> Result<usize> {
    read_lines(filename)?.fold(Ok(0), |num_valid, each_line| {
        let line = Line::parse(&each_line?)?;
        let chars: Vec<char> = line.password.chars().collect();
        let in_first = chars[(line.numbers.0 - 1) as usize] == line.letter;
        let in_second = chars[(line.numbers.1 - 1) as usize] == line.letter;
        if (in_first || in_second) && !(in_first && in_second) {
            Ok(num_valid? + 1)
        } else {
            num_valid
        }
    })
}
