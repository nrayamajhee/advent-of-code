use anyhow::Result;
use std::io;
use std::io::Write;
use aoc::year_2020::year_2020;

fn main() -> Result<()> {
    let mut invalid = false;
    loop {
        aoc::print_line!(
            "a: 2020",
            "b: 2020",
            "Pick a year: "
        );
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let choice = line.trim();
        match choice {
            "a" => {
                year_2020()?;
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
