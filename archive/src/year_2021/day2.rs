use anyhow::Result;
use crate::read_lines;

pub fn part1(filename: &str) -> Result<usize> {
    let (mut x, mut y) = (0,0);
    for line in read_lines(filename)? {
        let line = line?;
        let line: Vec<&str> = line.split(' ').collect();
        let inst = line[0]; 
        let value: isize = line[1].parse()?; 
        match inst {
            "forward" => x += value,
            "down" => y += value,
            "up" => y -= value,
            _ => ()
        }
    }
    Ok((x * y) as usize)
}

pub fn part2(filename: &str) -> Result<usize> {
    let (mut x, mut y) = (0,0);
    let (mut dx, mut dy) = (0,0);
    for line in read_lines(filename)? {
        let line = line?;
        let line: Vec<&str> = line.split(' ').collect();
        let inst = line[0]; 
        let value: isize = line[1].parse()?; 
        match inst {
            "forward" => {
                x += value;
                y += dy * value
            }
            "down" => dy += value,
            "up" => dy -= value,
            _ => ()
        }
    }
    Ok((x * y) as usize)
}
