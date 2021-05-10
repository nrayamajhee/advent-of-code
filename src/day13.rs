use crate::read_lines;
use anyhow::{Error, Result};

pub fn part1(filename: &str) -> Result<usize> {
    let mut read_head = read_lines(filename)?;
    let first = read_head
        .next()
        .ok_or_else(|| Error::msg("Invalid input file!"))??
        .parse::<usize>()?;
    let second = read_head
        .next()
        .ok_or_else(|| Error::msg("Invalid input file!"))??;
    let mut shortest = usize::MAX;
    let mut bus = None;
    for each in second.split(',') {
        if let Ok(number) = each.parse::<usize>() {
            let next = (first / number + 1) * number;
            if next < shortest {
                shortest = next;
                bus = Some(number);
            }
        }
    }
    if let Some(bus) = bus {
        Ok((shortest - first) * bus)
    } else {
        Err(Error::msg(
            "Couldn't find any bus that can take you to the airport!",
        ))
    }
}

/// Extended Euclidean Algorithm.
/// Refer to this video for explanation:
/// <https://www.youtube.com/watch?v=hB34-GSDT3k>.
///
/// For integers a and b this function returns (g, x, y) such that
/// g is the greatest common divisor and
/// g = a(x) + b(y)
fn ext_euclid_algo(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (r, fac_a, fac_b) = ext_euclid_algo(b, a % b);
        (r, fac_b, fac_a - (a / b) * fac_b)
    }
}

/// Uses Chinese Remainder Theorem to return output.
/// Refer to this video for explanation:
/// <https://www.youtube.com/watch?v=zIFehsBHB8o>
pub fn part2(filename: &str) -> Result<usize> {
    let numbers: Vec<(isize, isize)> = read_lines(filename)?
        .nth(1)
        .ok_or_else(|| Error::msg("Invalid input file!"))??
        .split(',')
        .enumerate()
        .filter_map(|(i, num)| num.parse().ok().map(|num| (num, (num - i as isize))))
        .collect();
    let prod = numbers.iter().map(|n| n.0).product::<isize>();
    let sum = numbers
        .iter()
        .map(|(num, rem)| {
            let pi = prod / num;
            let xi = ext_euclid_algo(pi, *num);
            // if the ext_euclid_algo return negative number add number and take mod again
            let xi = (xi.1 + num) % num;
            rem * pi * xi
        })
        .sum::<isize>();
    Ok((sum % prod) as usize)
}
