use anyhow::Result;
use std::io;
use std::io::Write;

fn main() -> Result<()> {
    let mut invalid = false;
    loop {
        aoc::print_line!(
            "Day1: Report Repair",
            "Day2: Password Philosophy",
            "Day3: Toboggan Trajectory",
            "Day4: Passport Processing",
            "Day5: Binary Boarding",
            "Day6: Custom Customs",
            "Day 7: Handy Haversacks",
            "Day 8: Handheld Halting",
            "Day 9: Encoding Error",
            "Day 9: Encoding Error",
            "Day 10: Adapter Array",
            "Day 11: Seating System",
            "Day 12: Rain Risk",
            "Day 13: Shuttle Search",
            "Day 14: Docking Data",
        );
        print!("Pick you day: ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let choice: u8 = line.trim().parse()?;
        match choice {
            1 => {
                use aoc::day1::*;
                let mut numbers = read_input("input/day1")?;
                numbers.sort_unstable();
                println!(
                    "The product of two numbers that sum up to 2020 is: {:?}",
                    part1("input/day1")?
                );
                println!(
                    "The product of three numbers that sum up to 2020 is: {:?}",
                    part2("input/day1")?
                );
            }
            2 => {
                use aoc::day2::*;
                println!("The number of valid passwords is: {}", part1("input/day2")?);
                println!(
                    "The number of officially valid passwords is: {}",
                    part2("input/day2")?
                );
            }
            3 => {
                use aoc::day3::*;
                println!(
                    "The number of trees encountered is: {}",
                    part1("input/day3")?
                );
                println!(
                    "The product of the trees encountered is: {}",
                    part2("input/day3")?
                );
            }
            4 => {
                use aoc::day4::*;
                println!("The number of valid passports is: {}", part1("input/day4")?);
                println!(
                    "The number of verified valid passports is: {}",
                    part2("input/day4")?
                );
            }
            5 => {
                use aoc::day5::*;
                println!("The biggest boarding id is: {}", part1("input/day5")?);
                println!("Your id is: {}", part2("input/day5")?);
            }
            6 => {
                use aoc::day6::*;
                println!(
                    "The total number of any yes answers is: {}",
                    part1("input/day6")?
                );
                println!(
                    "The total number of every yes answers is: {}",
                    part2("input/day6")?
                );
            }
            7 => {
                use aoc::day7::*;
                println!(
                    "The total number of ways to store shiny golden bag is: {}",
                    part1("input/day7")?
                );
                println!(
                    "The total number of bags that shiny golden bag contains is: {}",
                    part2("input/day7")?
                );
            }
            8 => {
                use aoc::day8::*;
                println!("The value of the accumulator is: {}", part1("input/day8")?);
                println!(
                    "The value of the accumulator is after correcting the corrupted instruction is: {}",
                    part2("input/day8")?
                );
            }
            9 => {
                use aoc::day9::*;
                println!(
                    "The first number that doesn't violates XMAS is: {}",
                    part1("input/day9", 25)?
                );
                println!("The sum of first and last of the contigous numbers that sum to the exception is: {}", part2("input/day9", 25)?);
            }
            10 => {
                use aoc::day10::*;
                println!(
                    "The number of 1-jolt differences multiplied by the number of 3-jolt differences is {}",
                    part1("input/day10")?
                );
                println!(
                    "The total number of distinct ways you can arrange the adapters to connect the charging outlet to your device is {}",
                    part2("input/day10")?
                );
            }
            11 => {
                use aoc::day11::*;
                println!("{} seats end up occupied", part1("input/day11")?);
                println!("{} seats end up occupied", part2("input/day11")?);
            }
            12 => {
                use aoc::day12::*;
                println!(
                    "The Manhattan distance between that location and the ship's starting position is {}",
                    part1("input/day12")?
                );
                println!(
                    "The Manhattan distance between that location and the ship's starting position is {}",
                    part2("input/day12")?
                );
            }
            13 => {
                use aoc::day13::*;
                println!(
                    "The ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus is {}",
                    part1("input/day13")?
                );
                println!(
                    "The ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus is {}",
                    part2("input/day13")?
                );
            }
            14 => {
                use aoc::day14::*;
                println!(
                    "The sum of all values left in memory after it completes is {}",
                    part1("input/day14")?
                );
                println!(
                    "The sum of all values left in memory after it completes is {}",
                    part2("input/day14")?
                );
            }
            _ => {
                invalid = true;
            }
        }
        if !invalid {
            break;
        }
    }
    Ok(())
}
