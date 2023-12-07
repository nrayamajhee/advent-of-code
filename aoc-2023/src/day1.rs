use aoc_runner_derive::aoc;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, e| {
        let numbers = e.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
        acc + format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap()
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let mut numbers = line
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if let Some(num) = c.to_digit(10) {
                    Some((i, num as usize))
                } else {
                    None
                }
            })
            .filter(|d| d.is_some())
            .map(|d| d.unwrap())
            .collect::<Vec<(usize, usize)>>();
        let found_words_left = DIGITS
            .iter()
            .enumerate()
            .map(|(num, word)| line.find(word).map(|i| (i, num + 1)))
            .filter(|d| d.is_some())
            .map(|d| d.unwrap())
            .collect::<Vec<(usize, usize)>>();
        let found_words_right = DIGITS
            .iter()
            .enumerate()
            .map(|(num, word)| line.rfind(word).map(|i| (i, num + 1)))
            .filter(|d| d.is_some())
            .map(|d| d.unwrap())
            .collect::<Vec<(usize, usize)>>();
        let mut words = found_words_left
            .iter()
            .chain(found_words_right.iter())
            .map(|i| *i)
            .collect::<Vec<(usize, usize)>>();
        numbers.append(&mut words);
        numbers.sort_by(|a, b| a.0.cmp(&b.0));
        acc + format!(
            "{}{}",
            numbers.first().unwrap().1,
            numbers.last().unwrap().1
        )
        .parse::<u32>()
        .unwrap()
    })
}
