#[cfg(test)]
mod day1 {
    use anyhow::Result;
    #[test]
    pub fn part1() -> Result<()> {
        use aoc::day1::*;
        let mut numbers = read_input("input/day1_test")?;
        numbers.sort_unstable();
        assert_eq!(part1(&numbers[..]).unwrap(), Some(514579));
        Ok(())
    }
    #[test]
    pub fn part2() -> Result<()> {
        use aoc::day1::*;
        let mut numbers = read_input("input/day1_test")?;
        numbers.sort_unstable();
        assert_eq!(part2(&numbers[..]).unwrap(), Some(241861950));
        Ok(())
    }
}

#[cfg(test)]
mod day2 {
    use anyhow::Result;
    #[test]
    pub fn part1() -> Result<()> {
        use aoc::day2::*;
        assert_eq!(2, part1("input/day2_test")?);
        Ok(())
    }
    #[test]
    pub fn part2() -> Result<()> {
        use aoc::day2::*;
        assert_eq!(1, part2("input/day2_test")?);
        Ok(())
    }
}
