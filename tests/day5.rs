#[cfg(test)]
mod day5 {
    use anyhow::Result;
    use aoc::day5::*;
    #[test]
    pub fn test_part1() -> Result<()> {
        assert_eq!(820, part1("input/day5_test")?);
        Ok(())
    }
}
