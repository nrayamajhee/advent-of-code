mod test_macro;

test! {
    (year_2020, day1, Some(514579), Some(241861950)),
    (year_2020, day2, 2, 1),
    (year_2020, day3, 7, 336),
    (year_2020, day6, 11, 6),
    (year_2020, day7, 4, 32),
    (year_2020, day8, 5, 8),
    (year_2020, day10, 220, 19208),
    (year_2020, day11, 37, 26),
    (year_2020, day12, 25, 286),
    (year_2020, day13, 295, 1068781),
}

#[cfg(test)]
mod day9 {
    use anyhow::Result;
    use aoc::year_2020::day9::*;
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
    (year_2020, day4, ("input/day4_test", 2),("input/day4_part2_test", 4)),
    (year_2020, day5, ("input/day5_test", 820),("input/day5", 527)),
    (year_2020, day14, ("input/day14_test", 165),("input/day14_test_2", 208))
}
