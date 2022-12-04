use std::ops::RangeInclusive;

fn parse_to_range(s: &str) -> RangeInclusive<i32> {
    let (start, end) = s.split_once('-').unwrap();
    start.parse::<i32>().unwrap()..=end.parse::<i32>().unwrap()
}

fn parse_line(l: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (e1, e2) = l.split_once(',').unwrap();
    (parse_to_range(e1), parse_to_range(e2))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|(r1, r2)| {
            (r1.start() >= r2.start() && r1.end() <= r2.end())
                || (r1.start() <= r2.start() && r1.end() >= r2.end())
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|(r1, r2)| !((r1.end() < r2.start()) || (r2.end() < r1.start())))
        .count()
}

fn main() {
    let input = include_str!("../input/day4.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day4.txt");

    #[test]
    fn parse_to_range_test() {
        assert_eq!(parse_to_range("2-4"), 2..=4)
    }
    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("1-3,2-4"), (1..=3, 2..=4))
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 4);
    }
}
