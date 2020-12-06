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
            "Day4: Passport Processing"
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
                    part1(&numbers[..])?
                );
                println!(
                    "The product of three numbers that sum up to 2020 is: {:?}",
                    part2(&numbers[..])?
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
