use anyhow::Result;
use crate::read_lines;

// This year, I'll read input into memory for the sake of
// a clean functional approach like below
// rather than cluncky for loops

fn read_lines_to_list(filename: &str) -> Result<Vec<usize>> {
    Ok(read_lines(filename)?
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect())
}

fn count_larger_numbers(numbers: Vec<usize>) -> usize {
    numbers.iter().enumerate().skip(1).fold(0, |accum, (i, e)| {
        if *e > numbers[i - 1] {
            accum + 1
        } else {
            accum
        }
    })
}

pub fn part1(filename: &str) -> Result<usize> {
    Ok(count_larger_numbers(read_lines_to_list(filename)?))
}

pub fn part2(filename: &str) -> Result<usize> {
    // forget about reducing memory usage and
    // create another huge list of sums
    let numbers = read_lines_to_list(filename)?;
    let sums: Vec<usize> = numbers
        .iter()
        .take(numbers.len() - 2)
        .enumerate()
        .map(|(i, each)| each + numbers[i + 1] + numbers[i + 2])
        .collect();
    Ok(count_larger_numbers(sums))
}
