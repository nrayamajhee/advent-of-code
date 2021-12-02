use crate::read_lines;
use anyhow::{Error, Result};
use std::collections::VecDeque;

pub fn part1(filename: &str, preamble: usize) -> Result<usize> {
    let mut buffer = VecDeque::new();
    let mut num = None;
    for line in read_lines(filename)? {
        let current = line?.parse::<usize>()?;
        if buffer.len() > preamble {
            buffer.pop_front();
        }
        if buffer.len() >= preamble {
            let mut temp_buf = buffer.clone();
            temp_buf.make_contiguous().sort_unstable();
            if super::day1::get_pairs_fast(temp_buf.as_slices().0, current) == None {
                num = Some(current);
                break;
            }
        }
        buffer.push_back(current);
    }
    if let Some(num) = num {
        Ok(num)
    } else {
        Err(Error::msg("No numbers found that defy the rule!"))
    }
}
pub fn part2(filename: &str, preamble: usize) -> Result<usize> {
    let invalid_num = part1(filename, preamble)?;
    let mut buffer = VecDeque::new();
    for line in read_lines(filename)? {
        let current = line?.parse::<usize>()?;
        buffer.push_back(current);
        let sum: usize = buffer.iter().sum();
        if sum > invalid_num {
            loop {
                buffer.pop_front();
                let new_sum: usize = buffer.iter().sum();
                if new_sum < invalid_num {
                    break;
                }
                if new_sum == invalid_num {
                    buffer.make_contiguous().sort_unstable();
                    let (first, last) = (
                        buffer
                            .pop_front()
                            .ok_or_else(|| anyhow::Error::msg("Empty Buffer"))?,
                        buffer
                            .pop_back()
                            .ok_or_else(|| anyhow::Error::msg("Empty Buffer"))?,
                    );
                    return Ok(first + last);
                }
            }
        }
    }
    Err(Error::msg(
        "No contiguous numbers found that sum to the exception!",
    ))
}
