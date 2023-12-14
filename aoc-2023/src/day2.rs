use aoc_runner_derive::aoc;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Draw {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split(",")
            .map(|s| s.trim().split(" ").collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let parse_color = |color| {
            let value = split
                .iter()
                .filter(|s| s[1] == color)
                .map(|s| s[0].parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if value.is_empty() {
                0
            } else {
                value[0]
            }
        };
        let red = parse_color("red");
        let green = parse_color("green");
        let blue = parse_color("blue");
        Ok(Draw { red, green, blue })
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    let cubes = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    input.lines().fold(0, |acc, line| {
        let split = line.split(":").collect::<Vec<_>>();
        let game_id = split[0].split(" ").collect::<Vec<_>>()[1]
            .parse::<usize>()
            .unwrap();
        let draws = split[1]
            .split(";")
            .map(|s| s.parse::<Draw>().unwrap())
            .collect::<Vec<_>>();
        let valid = draws
            .iter()
            .all(|d| d.red <= cubes.red && d.green <= cubes.green && d.blue <= cubes.blue);
        if valid {
            acc + game_id
        } else {
            acc
        }
    })
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let split = line.split(":").collect::<Vec<_>>();
        let draws = split[1]
            .split(";")
            .map(|s| s.parse::<Draw>().unwrap())
            .collect::<Vec<_>>();
        let least_draw = draws.iter().fold(
            Draw {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, draw| Draw {
                red: if draw.red > acc.red {
                    draw.red
                } else {
                    acc.red
                },
                green: if draw.green > acc.green {
                    draw.green
                } else {
                    acc.green
                },
                blue: if draw.blue > acc.blue {
                    draw.blue
                } else {
                    acc.blue
                },
            },
        );
        let power = least_draw.red * least_draw.green * least_draw.blue;
        acc + power
    })
}
