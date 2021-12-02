#[macro_export]
macro_rules! test_file {
    ($(($year: ident, $day:ident, ($file1: expr, $ans1: expr), ($file2: expr, $ans2: expr))),+$(,)?) => {
        $(
            #[cfg(test)]
            mod $day {
                use anyhow::Result;
                use aoc::$year::$day::*;
                #[test]
                pub fn test_part1() -> Result<()> {
                    assert_eq!($ans1, part1($file1)?);
                    Ok(())
                }
                #[test]
                pub fn test_part2() -> Result<()> {
                    assert_eq!($ans2, part2($file2)?);
                    Ok(())
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! test {
    ($(($year: ident, $day:ident, $ans1: expr, $ans2: expr)),+$(,)?) => {
        $(
            test_file!{
                (
                    $year,
                    $day,
                    (&format!("input/{}_test", stringify!($day)), $ans1),
                    (&format!("input/{}_test", stringify!($day)), $ans2)
                )
             }
        )*
    }
}
