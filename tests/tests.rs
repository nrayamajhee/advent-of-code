#[cfg(test)]
mod day1 {
    use anyhow::Result;
    use aoc::day1::*;
    #[test]
    pub fn test_part1() -> Result<()> {
        let mut numbers = read_input("input/day1_test")?;
        numbers.sort_unstable();
        assert_eq!(part1(&numbers[..]).unwrap(), Some(514579));
        Ok(())
    }
    #[test]
    pub fn test_part2() -> Result<()> {
        let mut numbers = read_input("input/day1_test")?;
        numbers.sort_unstable();
        assert_eq!(part2(&numbers[..]).unwrap(), Some(241861950));
        Ok(())
    }
}

#[cfg(test)]
mod day2 {
    use anyhow::Result;
    use aoc::day2::*;
    #[test]
    pub fn test_part1() -> Result<()> {
        assert_eq!(2, part1("input/day2_test")?);
        Ok(())
    }
    #[test]
    pub fn test_part2() -> Result<()> {
        assert_eq!(1, part2("input/day2_test")?);
        Ok(())
    }
}

#[cfg(test)]
mod day3 {
    use anyhow::Result;
    use aoc::day3::*;
    #[test]
    pub fn test_part1() -> Result<()> {
        assert_eq!(7, part1("input/day3_test")?);
        Ok(())
    }
    #[test]
    pub fn test_part2() -> Result<()> {
        assert_eq!(336, part2("input/day3_test")?);
        Ok(())
    }
}

#[cfg(test)]
mod day4 {
    use anyhow::Result;
    use aoc::day4::*;
    #[test]
    pub fn test_part1() -> Result<()> {
        assert_eq!(2, part1("input/day4_test")?);
        Ok(())
    }
    #[test]
    pub fn test_part2() -> Result<()> {
        assert_eq!(4, part2("input/day4_part2_test")?);
        Ok(())
    }
}
