use crate::read_lines;
use anyhow::Result;

pub fn get_pairs(numbers: &[u16], sum: u16) -> Option<(u16, u16)> {
    for each in numbers {
        if sum > *each {
            let another = sum - each;
            if let Ok(_) = numbers.binary_search(&another) {
                return Some((*each, another));
            }
        }
    }
    None
}

pub fn get_pairs_fast(numbers: &[u16], sum: u16) -> Option<(u16, u16)> {
    // use two indices to solve in O(n)
    unimplemented!();
}

pub fn get_triplets(numbers: &[u16], sum: u16) -> Option<(u16, u16, u16)> {
    for each in numbers {
        if sum > *each {
            let remainder = sum - each;
            if let Some((a, b)) = get_pairs(numbers, remainder) {
                return Some((*each, a, b));
            }
        }
    }
    None
}

pub fn get_triplets_fast(numbers: &[u16], sum: u16) -> Option<(u16, u16, u16)> {
    // use hash set to solve in O(n^2)
    unimplemented!();
}

pub fn read_input(filename: &str) -> Result<Vec<u16>> {
    let mut numbers = Vec::new();
    for line in read_lines(filename)? {
        if let Ok(num) = line?.parse::<u16>() {
            numbers.push(num);
        }
    }
    Ok(numbers)
}

const SUM: u16 = 2020;

pub fn part1(numbers: &[u16]) -> Result<Option<u32>> {
    let prod2 = if let Some((a, b)) = get_pairs(&numbers[..], SUM) {
        Some(a as u32 * b as u32)
    } else {
        None
    };
    Ok(prod2)
}

pub fn part2(numbers: &[u16]) -> Result<Option<u32>> {
    let prod3 = if let Some((a, b, c)) = get_triplets(&numbers[..], SUM) {
        Some(a as u32 * b as u32 * c as u32)
    } else {
        None
    };
    Ok(prod3)
}
