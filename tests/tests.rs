mod test_macro;

test! {
    (day1, Some(514579), Some(241861950)),
    (day2, 2, 1),
    (day3, 7, 336),
    (day6, 11, 6),
    (day7, 4, 32),
    (day8, 5, 8),
    (day10, 220, 19208),
    (day11, 37, 26),
    (day12, 25, 286),
}

#[cfg(test)]
mod day9 {
    use anyhow::Result;
    use aoc::day9::*;
    #[test]
    pub fn test_part1() -> Result<()> {
        assert_eq!(part1("input/day9_test", 5)?, 127);
        Ok(())
    }
    #[test]
    pub fn test_part2() -> Result<()> {
        assert_eq!(part2("input/day9_test", 5)?, 62);
        Ok(())
    }
}

test_file! {
    (day4, ("input/day4_test", 2),("input/day4_part2_test", 4)),
    (day5, ("input/day5_test", 820),("input/day5", 527))
}
