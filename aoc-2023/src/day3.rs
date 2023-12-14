use aoc_runner_derive::aoc;

#[derive(PartialEq)]
enum Data {
    Num,
    Sym,
    Dot,
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut previous_char = Data::Dot;
    let mut number_buffer = String::new();
    let mut position: Option<(usize, usize, usize)> = None;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.to_digit(10).is_some() {
                number_buffer.push_str(&c.to_string());
                if previous_char == Data::Num {
                    if let Some(ref mut pos) = position {
                        *pos = (pos.0, pos.1, x);
                    }
                } else {
                    position = Some((y, x, x));
                }
                previous_char = Data::Num;
            } else {
                if previous_char == Data::Num {
                    numbers.push((number_buffer.parse::<usize>().unwrap(), position.unwrap()));
                    number_buffer.clear();
                }
                if c != '.' {
                    symbols.push((c, y, x));
                    previous_char = Data::Sym;
                } else {
                    previous_char = Data::Dot;
                }
            }
        }
    }
    dbg!(&numbers, symbols);
    0
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    0
}
