use anyhow::Result;
use std::io;
use std::io::Write;

crate::use_mod!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14);

pub fn year_2020() -> Result<()> {
    loop {
        crate::print_line!(
            "Day  1: Report Repair",
            "Day  2: Password Philosophy",
            "Day  3: Toboggan Trajectory",
            "Day  4: Passport Processing",
            "Day  5: Binary Boarding",
            "Day  6: Custom Customs",
            "Day  7: Handy Haversacks",
            "Day  8: Handheld Halting",
            "Day  9: Encoding Error",
            "Day  9: Encoding Error",
            "Day 10: Adapter Array",
            "Day 11: Seating System",
            "Day 12: Rain Risk",
            "Day 13: Shuttle Search",
            "Day 14: Docking Data",
            "Any other number/letter to go back"
        );
        print!("Pick your day: ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let choice = line.trim().parse::<u8>();
        if let Ok(choice) = choice {
            match choice {
                1 => {
                    use day1::*;
                    let mut numbers = read_input("input/year_2020/day1")?;
                    numbers.sort_unstable();
                    println!(
                        "The product of two numbers that sum up to 2020 is: {:?}",
                        part1("input/year_2020/day1")?
                    );
                    println!(
                        "The product of three numbers that sum up to 2020 is: {:?}",
                        part2("input/year_2020/day1")?
                    );
                }
                2 => {
                    use day2::*;
                    println!("The number of valid passwords is: {}", part1("input/year_2020/day2")?);
                    println!(
                        "The number of officially valid passwords is: {}",
                        part2("input/year_2020/day2")?
                    );
                }
                3 => {
                    use day3::*;
                    println!(
                        "The number of trees encountered is: {}",
                        part1("input/year_2020/day3")?
                    );
                    println!(
                        "The product of the trees encountered is: {}",
                        part2("input/year_2020/day3")?
                    );
                }
                4 => {
                    use day4::*;
                    println!("The number of valid passports is: {}", part1("input/year_2020/day4")?);
                    println!(
                        "The number of verified valid passports is: {}",
                        part2("input/year_2020/day4")?
                    );
                }
                5 => {
                    use day5::*;
                    println!("The biggest boarding id is: {}", part1("input/year_2020/day5")?);
                    println!("Your id is: {}", part2("input/year_2020/day5")?);
                }
                6 => {
                    use day6::*;
                    println!(
                        "The total number of any yes answers is: {}",
                        part1("input/year_2020/day6")?
                    );
                    println!(
                        "The total number of every yes answers is: {}",
                        part2("input/year_2020/day6")?
                    );
                }
                7 => {
                    use day7::*;
                    println!(
                        "The total number of ways to store shiny golden bag is: {}",
                        part1("input/year_2020/day7")?
                    );
                    println!(
                        "The total number of bags that shiny golden bag contains is: {}",
                        part2("input/year_2020/day7")?
                    );
                }
                8 => {
                    use day8::*;
                    println!("The value of the accumulator is: {}", part1("input/year_2020/day8")?);
                    println!(
                        "The value of the accumulator is after correcting the corrupted instruction is: {}",
                        part2("input/year_2020/day8")?
                    );
                }
                9 => {
                    use day9::*;
                    println!(
                        "The first number that doesn't violates XMAS is: {}",
                        part1("input/year_2020/day9", 25)?
                    );
                    println!("The sum of first and last of the contigous numbers that sum to the exception is: {}", part2("input/year_2020/day9", 25)?);
                }
                10 => {
                    use day10::*;
                    println!(
                        "The number of 1-jolt differences multiplied by the number of 3-jolt differences is {}",
                        part1("input/year_2020/day10")?
                    );
                    println!(
                        "The total number of distinct ways you can arrange the adapters to connect the charging outlet to your device is {}",
                        part2("input/year_2020/day10")?
                    );
                }
                11 => {
                    use day11::*;
                    println!("{} seats end up occupied", part1("input/year_2020/day11")?);
                    println!("{} seats end up occupied", part2("input/year_2020/day11")?);
                }
                12 => {
                    use day12::*;
                    println!(
                        "The Manhattan distance between that location and the ship's starting position is {}",
                        part1("input/year_2020/day12")?
                    );
                    println!(
                        "The Manhattan distance between that location and the ship's starting position is {}",
                        part2("input/year_2020/day12")?
                    );
                }
                13 => {
                    use day13::*;
                    println!(
                        "The ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus is {}",
                        part1("input/year_2020/day13")?
                    );
                    println!(
                        "The ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus is {}",
                        part2("input/year_2020/day13")?
                    );
                }
                14 => {
                    use day14::*;
                    println!(
                        "The sum of all values left in memory after it completes is {}",
                        part1("input/year_2020/day14")?
                    );
                    println!(
                        "The sum of all values left in memory after it completes is {}",
                        part2("input/year_2020/day14")?
                    );
                }
                _ => {
                    break;
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}
