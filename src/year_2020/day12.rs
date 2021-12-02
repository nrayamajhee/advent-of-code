use crate::read_lines;
use anyhow::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Movement {
    Forward,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Line {
    Dir(Direction, usize),
    Mov(Movement, usize),
}

impl Line {
    fn parse(line: &str) -> Result<Self> {
        let value = line[1..].parse::<usize>()?;
        if let Ok(dir) = Direction::try_from(&line[0..1]) {
            return Ok(Line::Dir(dir, value));
        }
        if let Ok(rot) = Movement::try_from(&line[0..1]) {
            return Ok(Line::Mov(rot, value));
        }
        Err(Error::msg("Couldn't parse line"))
    }
}

use std::convert::TryFrom;

impl Direction {
    fn rotate(self, rot: Movement, deg: usize) -> Self {
        let deg = match rot {
            Movement::Forward => 0,
            Movement::Left => -(deg as isize),
            Movement::Right => deg as isize,
        };
        let dr = deg % 360 / 90;
        let mut new_dir = (self as isize + dr) % 4;
        if new_dir < 0 {
            new_dir += 4;
        }
        match new_dir.abs() {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "N" => Ok(Direction::North),
            "E" => Ok(Direction::East),
            "W" => Ok(Direction::West),
            "S" => Ok(Direction::South),
            _ => Err(anyhow::Error::msg("Invalid direction character")),
        }
    }
}

impl TryFrom<&str> for Movement {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "L" => Ok(Movement::Left),
            "R" => Ok(Movement::Right),
            "F" => Ok(Movement::Forward),
            _ => Err(anyhow::Error::msg("Invalid direction character")),
        }
    }
}

pub fn part1(filename: &str) -> Result<usize> {
    let mut facing = Direction::East;
    let mut coordinates = (0, 0);
    for each in read_lines(filename)? {
        let line = Line::parse(&each?)?;
        let (mut dx, mut dy) = (0, 0);
        let translate = |dir, value: isize| {
            let dx = match dir {
                Direction::East => value,
                Direction::West => -value,
                _ => 0,
            };
            let dy = match dir {
                Direction::North => value,
                Direction::South => -value,
                _ => 0,
            };
            (dx, dy)
        };
        match line {
            Line::Dir(dir, value) => {
                let t = translate(dir, value as isize);
                dx = t.0;
                dy = t.1;
            }
            Line::Mov(rot, value) => {
                if rot == Movement::Forward {
                    let t = translate(facing, value as isize);
                    dx = t.0;
                    dy = t.1;
                } else {
                    facing = facing.rotate(rot, value as usize);
                }
            }
        }
        let (x, y) = coordinates;
        coordinates = (x + dx, y + dy);
    }
    Ok((coordinates.0.abs() + coordinates.1.abs()) as usize)
}

fn rotate(coord: (isize, isize), deg: isize) -> (isize, isize) {
    let x = coord.0 as f32;
    let y = coord.1 as f32;
    let deg = deg as f32 / 180. * std::f32::consts::PI;
    let n_x = x * deg.cos() - y * deg.sin();
    let n_y = x * deg.sin() + y * deg.cos();
    (n_x.round() as isize, n_y.round() as isize)
}

pub fn part2(filename: &str) -> Result<usize> {
    let mut coordinates = (0, 0);
    let mut waypoint = (10, 1);
    for each in read_lines(filename)? {
        let line = Line::parse(&each?)?;
        match line {
            Line::Dir(dir, value) => {
                let value = value as isize;
                let dx = match dir {
                    Direction::East => value,
                    Direction::West => -value,
                    _ => 0,
                };
                let dy = match dir {
                    Direction::North => value,
                    Direction::South => -value,
                    _ => 0,
                };
                waypoint.0 += dx;
                waypoint.1 += dy;
            }
            Line::Mov(rot, value) => {
                let value = value as isize;
                if rot == Movement::Forward {
                    let dx = value * waypoint.0;
                    let dy = value * waypoint.1;
                    let (x, y) = coordinates;
                    coordinates = (x + dx, y + dy);
                } else {
                    let value = if rot == Movement::Right {
                        -value
                    } else {
                        value
                    };
                    waypoint = rotate(waypoint, value);
                }
            }
        }
    }
    Ok((coordinates.0.abs() + coordinates.1.abs()) as usize)
}
