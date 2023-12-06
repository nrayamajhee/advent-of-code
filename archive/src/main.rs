use anyhow::Result;
use std::io;
use std::io::Write;
use aoc::year_2020::year_2020;
use aoc::year_2021::year_2021;

fn main() -> Result<()> {
    loop {
        aoc::print_line!(
            "a: 2021",
            "b: 2020",
            "Any other letter to quit"
        );
        print!("Pick a year: ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let choice = line.trim();
        match choice {
            "a" => {
                year_2021()?;
            }
            "b" => {
                year_2020()?;
            }
            _ => {
                break;
            }
        }
    }
    Ok(())
}
