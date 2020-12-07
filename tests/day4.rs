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
