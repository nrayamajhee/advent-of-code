use crate::read_lines;
use anyhow::Result;
use std::collections::HashMap;

pub fn part1(filename: &str) -> Result<usize> {
    let mut counts: Vec<HashMap<char, usize>> = Vec::new();
    for line in read_lines(filename)? {
        for (i, digit) in line?.chars().enumerate() {
            if let Some(map) = counts.get_mut(i) {
                if let Some(value) = map.get_mut(&digit) {
                    *value += 1;
                }
            } else {
                let mut map = HashMap::new();
                map.insert('0', if digit == '0' { 1 } else { 0 });
                map.insert('1', if digit == '1' { 1 } else { 0 });
                counts.push(map);
            }
        }
    }
    let (gamma, epsilon) =
        counts
            .iter()
            .fold((String::new(), String::new()), |(gamma, epsilon), each| {
                let big = each.iter().fold(
                    '0',
                    |big, (k, v)| {
                        if v > each.get(&big).unwrap() {
                            *k
                        } else {
                            big
                        }
                    },
                );
                let small = each.iter().fold('1', |small, (k, v)| {
                    if v < each.get(&big).unwrap() {
                        *k
                    } else {
                        small
                    }
                });
                (format!("{}{}", gamma, big), format!("{}{}", epsilon, small))
            });
    let gamma = usize::from_str_radix(&gamma, 2)?;
    let epsilon = usize::from_str_radix(&epsilon, 2)?;
    Ok(gamma * epsilon)
}

pub fn part2(filename: &str) -> Result<usize> {
    Ok(0)
}
