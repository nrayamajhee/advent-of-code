mod test_macro;

test! {
    (day1, Some(514579), Some(241861950)),
    (day2, 2, 1),
    (day3, 7, 336),
    (day6, 11, 6),
    (day7, 4, 32),
    (day8, 5, 8),
}

test_file! {(day4, ("input/day4_test", 2), ("input/day4_part2_test", 4))}
test_file! {(day5, ("input/day5_test", 820), ("input/day5", 527))}
