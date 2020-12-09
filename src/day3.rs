use crate::read_lines;
use anyhow::Result;

pub fn traverse(filename: &str, right: usize, down: usize) -> Result<usize> {
    let (_, trees) =
        read_lines(filename)?
            .step_by(down)
            .fold((0, 0), |(mut left, mut trees), line| {
                if let Ok(line) = line {
                    if &line[left..left + 1] == "#" {
                        trees += 1;
                    }
                    left = (left + right) % line.len();
                }
                (left, trees)
            });
    Ok(trees)
}

pub fn part1(filename: &str) -> Result<usize> {
    traverse(filename, 3, 1)
}

pub fn part2(filename: &str) -> Result<usize> {
    let combination = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;
    for (right, down) in combination.iter() {
        product *= traverse(filename, *right, *down)?;
    }
    Ok(product)
}
