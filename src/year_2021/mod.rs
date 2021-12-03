use anyhow::Result;
use std::io;
use std::io::Write;

crate::use_mod!(day1, day2, day3);

pub fn year_2021() -> Result<()> {
    loop {
        crate::print_line!(
            "Day 1: Sonar Sweep",
            "Day 2: Dive!",
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
                    println!(
                        "The number of measurements larger than the last one is: {:?}",
                        part1("input/year_2021/day1")?
                    );
                    println!(
                        "The number of measurements larger than the last one is: {:?}",
                        part2("input/year_2021/day1")?
                    );
                }
                2 => {
                    use day2::*;
                    println!(
                        "The product of the final horizontal and vertical position is {}",
                        part1("input/year_2021/day2")?
                    );
                    println!(
                        "The product of the final horizontal and vertical position is {}",
                        part2("input/year_2021/day2")?
                    );
                }
                3 => {
                    use day3::*;
                    println!(
                        "The power consumption of the submarine is {}.",
                        part1("input/year_2021/day3")?
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
