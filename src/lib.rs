use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::env;

pub fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let mut path = env::current_dir()?;
    path.push(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub mod day1;
pub mod day2;
