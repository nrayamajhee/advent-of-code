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
