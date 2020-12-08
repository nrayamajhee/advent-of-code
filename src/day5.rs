use crate::read_lines;
use anyhow::{Error, Result};

#[allow(dead_code)]
fn find_id_shift(code: &str) -> Result<usize> {
    let mut row = 0..127;
    let mut col = 0..7;
    for each in code.chars() {
        let mr = ((row.end - row.start) / 2) + 1;
        let mc = ((col.end - col.start) / 2) + 1;
        match &each {
            'B' => row.start += mr,
            'R' => col.start += mc,
            'F' => row.end -= mr,
            'L' => col.end -= mc,
            _ => (),
        }
    }
    if row.start == row.end && col.start == col.end {
        Ok(row.start * 8 + col.start)
    } else {
        Err(Error::msg("Not a valid id!"))
    }
}

fn find_id_binary(code: &str) -> usize {
    code.chars()
        .fold(0, |acc, c| acc << 1 | matches!(c, 'B' | 'R') as usize)
}

pub fn part1(filename: &str) -> Result<usize> {
    let mut biggest_id = None;
    for each in read_lines(filename)? {
        let new_id = find_id_binary(&each?);
        let bigger = if let Some(bid) = biggest_id {
            new_id > bid
        } else {
            true
        };
        if bigger {
            biggest_id = Some(new_id);
        }
    }
    if let Some(bid) = biggest_id {
        Ok(bid)
    } else {
        Err(Error::msg("No valid ids found in the list!"))
    }
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut ids = Vec::new();
    for each in read_lines(filename)? {
        ids.push(find_id_binary(&each?));
    }
    ids.sort_unstable();
    let mut your_id = None;
    for (i, each) in ids.iter().enumerate() {
        if i < ids.len() - 2 {
            let next = ids[i + 1];
            if next != each + 1 && next == each + 2 {
                your_id = Some(*each + 1);
            }
        }
    }
    if let Some(id) = your_id {
        Ok(id)
    } else {
        Err(Error::msg("Couldn't find your id in the list!"))
    }
}
