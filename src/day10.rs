use crate::read_lines;
use anyhow::Result;

pub fn part1(filename: &str) -> Result<usize> {
    let mut buffer = vec![0];
    for each in read_lines(filename)? {
        buffer.push(each?.parse::<usize>()?);
    }
    buffer.sort_unstable();
    buffer.push(buffer[buffer.len() - 1] + 3);
    let mut num_one = 0;
    let mut num_three = 0;
    for (i, each) in buffer.iter().enumerate() {
        if i < buffer.len() - 1 {
            let next = buffer[i + 1];
            if next - each == 1 {
                num_one += 1;
            } else if next - each == 3 {
                num_three += 1;
            }
        }
    }
    Ok((num_one) * (num_three))
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut memo = vec![(0, 1)];
    for each in read_lines(filename)? {
        let each = each?.parse::<usize>()?;
        memo.push((each, 0));
    }
    memo.sort_by_key(|(k, _)| *k);
    for i in 1..memo.len() {
        let mut new_count = 0;
        for j in i as isize -3..i as isize {
            if j >= 0 {
                let (prev, p_count) = memo[j as usize];
                if memo[i].0 - prev <= 3 {
                    new_count += p_count;
                }
            }
        }
        memo[i].1 = new_count;
    }
    Ok(memo[memo.len() - 1].1)
}
