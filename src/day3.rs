use crate::read_lines;
use anyhow::Result;

pub fn traverse(filename: &str, right: usize, down: usize) -> Result<usize> {
    let mut trees = 0;
    let mut left = 0;
    for (i, each) in read_lines(filename)?.enumerate() {
        if i % down == 0 {
            let line: Vec<char> = each?.chars().collect();
            let index = left % line.len();
            if line[index] == '#' {
                trees += 1;
            }
            left += right;
        }
    }
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
