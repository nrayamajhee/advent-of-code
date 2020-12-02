use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn get_pairs(numbers: &[u16], sum: u16) -> Option<(u16, u16)> {
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

fn get_triplets(numbers: &[u16], sum: u16) -> Option<(u16, u16, u16)> {
    for each in numbers {
        if sum > *each {
            let remainder = sum - each;
            if let Some((a,b)) = get_pairs(numbers, remainder) {
                return Some((*each, a, b));
            }
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let mut path = env::current_dir()?;
    path.push("puzzle");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();
    for line in reader.lines() {
        if let Ok(num) = line {
            if let Ok(num) = num.parse::<u16>() {
                numbers.push(num);
            }
        }
    }
    numbers.sort_unstable();
    let sum = 2020;
    if let Some(p) = get_pairs(&numbers[..], sum) {
        println!("Product of two is {}", (p.0 as u32 * p.1 as u32));
    }
    if let Some(p) = get_triplets(&numbers[..], sum) {
        println!("Product of three is {}", (p.0 as u32 * p.1 as u32 * p.2 as u32));
    }
    Ok(())
}
