use crate::read_lines;
use anyhow::{Error, Result};
use std::collections::BTreeMap;

pub fn part1(filename: &str) -> Result<usize> {
    let mut mask = String::new();
    // Using BTree just so that the memory is sorted by location
    let mut memory = BTreeMap::new();
    for line in read_lines(filename)? {
        let line = line?;
        let line: Vec<_> = line.split(" = ").collect();
        if line[0] == "mask" {
            mask = line[1].to_string();
        } else {
            let mut ones = 0;
            let mut zeros = -1;
            let mem: Vec<_> = line[0].split(['[', ']'].as_ref()).collect();
            let loc = mem[1].parse::<usize>()?;
            // Bit Manipulation requires this to be isize and not usize
            // we will later store it as usize on the map
            let mut value = line[1].parse::<isize>()?;
            for (i, c) in mask.chars().enumerate() {
                let i_right = (mask.len() - i - 1) as isize;
                match c {
                    '1' => {
                        ones |= 1 << i_right;
                    }
                    '0' => {
                        zeros &= !(1 << i_right);
                    }
                    _ => (),
                }
            }
            value &= zeros;
            value |= ones;
            memory.insert(loc, value as usize);
        }
    }
    let sum = memory.iter().map(|(_, v)| *v).sum();
    Ok(sum)
}

fn gen_combos(value: &str) -> Result<Vec<isize>> {
    let mut combs: Vec<isize> = Vec::new();
    combs.push(isize::from_str_radix(&value.replace("X", "0"), 2)?);
    for (i, each) in value.chars().rev().enumerate() {
        if each == 'X' {
            let mut small_comb = Vec::new();
            for each in combs.iter() {
                let new = each | (1 << i);
                small_comb.push(new);
            }
            combs.append(&mut small_comb);
        }
    }
    Ok(combs)
}

fn char_to_u8(character: char) -> Result<u8> {
    u8::from_str_radix(&character.to_string(), 2).or(Err(Error::msg("Invalid character provided!")))
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut mask = String::new();
    let mut memory = BTreeMap::new();
    for line in read_lines(filename)? {
        let line = line?;
        let line: Vec<_> = line.split(" = ").collect();
        if line[0] == "mask" {
            mask = line[1].to_string();
        } else {
            let mut masked_loc = String::new();
            let mem: Vec<_> = line[0].split(['[', ']'].as_ref()).collect();
            let loc = format!("{:0width$b}", mem[1].parse::<isize>()?, width = mask.len());
            for (i, each) in mask.chars().enumerate() {
                if each != 'X' {
                    let first = char_to_u8(each)?;
                    let second = if let Some(bit) = loc.chars().nth(i) {
                        char_to_u8(bit)?
                    } else {
                        0
                    };
                    let bit = first | second;
                    masked_loc.push_str(&format!("{}", bit));
                } else {
                    masked_loc.push(each);
                }
            }
            let value = line[1].parse::<usize>()?;
            let combinations = gen_combos(&masked_loc)?;
            for each in combinations {
                memory.insert(each, value);
            }
        }
    }
    let sum = memory.iter().map(|(_, v)| *v).sum();
    Ok(sum)
}
