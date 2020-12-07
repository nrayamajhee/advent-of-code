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

