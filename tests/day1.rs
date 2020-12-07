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
