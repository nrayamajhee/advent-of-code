use crate::read_lines;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn part1(filename: &str) -> Result<usize> {
    let mut answers = HashSet::new();
    let mut sum = 0;
    for each in read_lines(filename)? {
        let line = each?;
        if line == "" {
            sum += answers.len();
            answers.clear();
        } else {
            for each in line.chars() {
                answers.insert(each);
            }
        }
    }
    // check again because the input might not have a new line at the end
    if answers.len() > 0 {
        sum += answers.len()
    }
    Ok(sum)
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut answers = HashMap::new();
    let mut sum = 0;
    let mut num_person = 0;
    let mut calculate = |answers: &mut HashMap<char, usize>, num_person| {
        sum += answers.iter().fold(
            0,
            |acc, (_, v)| {
                if *v == num_person {
                    acc + 1
                } else {
                    acc
                }
            },
        );
        answers.clear();
    };
    for each in read_lines(filename)? {
        let line = each?;
        if line == "" {
            calculate(&mut answers, num_person);
            num_person = 0;
        } else {
            for each in line.chars() {
                if let Some(count) = answers.get_mut(&each) {
                    *count += 1;
                } else {
                    answers.insert(each, 1);
                }
            }
            num_person += 1;
        }
    }
    // check again because the input might not have a new line at the end
    if answers.len() > 0 {
        calculate(&mut answers, num_person);
    }
    Ok(sum)
}
