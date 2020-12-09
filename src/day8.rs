use crate::read_lines;
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Nop(isize),
    Jmp(isize),
    Acc(isize),
}

impl Instruction {
    fn parse(line: &str) -> Result<Self> {
        let split: Vec<_> = line.split(" ").collect();
        let number = split[1].parse()?;
        match split[0] {
            "nop" => Ok(Self::Nop(number)),
            "jmp" => Ok(Self::Jmp(number)),
            "acc" => Ok(Self::Acc(number)),
            _ => Err(Error::msg("Not a valid instruction!")),
        }
    }
}

fn loop_until_cyclic_or_end(lines: &Vec<Instruction>) -> Result<(bool, isize)> {
    let mut unique_lines = HashSet::new();
    let mut acc = 0;
    let mut ip = 0;
    let mut cyclic = false;
    loop {
        if ip >= lines.len() {
            break;
        }
        let line = lines[ip];
        if unique_lines.contains(&ip) {
            cyclic = true;
            break;
        } else {
            unique_lines.insert(ip);
            if let Instruction::Jmp(line) = line {
                let nxt_ptr = ip as isize + line;
                if nxt_ptr >= 0 {
                    ip = nxt_ptr as usize;
                } else {
                    return Err(Error::msg(
                        "The jmp instruction wants to go to negative lines!",
                    ));
                }
            } else {
                if let Instruction::Acc(value) = line {
                    acc += value;
                }
                ip += 1;
            }
        }
    }
    Ok((cyclic, acc))
}

pub fn part1(filename: &str) -> Result<isize> {
    let mut lines = Vec::new();
    for each in read_lines(filename)? {
        lines.push(Instruction::parse(&each?)?);
    }
    let (_, acc) = loop_until_cyclic_or_end(&lines)?;
    Ok(acc)
}
pub fn part2(filename: &str) -> Result<isize> {
    let mut lines = Vec::new();
    for each in read_lines(filename)? {
        lines.push(Instruction::parse(&each?)?);
    }
    let mut tested = HashSet::new();
    let acc = loop {
        let mut test_lines = lines.clone();
        for (i, each) in test_lines.iter_mut().enumerate() {
            if !tested.contains(&i) {
                let next = match each {
                    Instruction::Nop(line) => Some(Instruction::Jmp(*line)),
                    Instruction::Jmp(line) => Some(Instruction::Nop(*line)),
                    _ => None,
                };
                if let Some(nxt) = next {
                    *each = nxt;
                    tested.insert(i);
                    break;
                }
            }
        }
        if let Ok((false, valid_acc)) = loop_until_cyclic_or_end(&test_lines) {
            break valid_acc;
        }
    };
    Ok(acc)
}
