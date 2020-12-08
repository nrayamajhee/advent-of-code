use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let mut path = env::current_dir()?;
    path.push(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

#[macro_export]
macro_rules! use_mod {
    ($($mod:ident),+$(,)?) => {
        $(
            pub mod $mod;
        )*
    }
}

#[macro_export]
macro_rules! print_line {
    ($($line:expr),+$(,)?) => {
        let mut string = String::new();
        $(
            string.push_str(&$line);
            string.push('\n');
        )*
        println!("{}", &string);
    }
}

use_mod!(day1, day2, day3, day4, day5, day6);
