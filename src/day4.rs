type Assignment = (i32, i32);
fn parse_to_assignment(s: &str) -> Assignment {
    let (s, e) = s.split_once('-').unwrap();
    (s.parse().unwrap(), e.parse().unwrap())
}

fn parse_line(l: &str) -> (Assignment, Assignment) {
    let (a1, a2) = l.split_once(',').unwrap();
    (parse_to_assignment(a1), parse_to_assignment(a2))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|((s1, e1), (s2, e2))| (s1 >= s2 && e1 <= e2) || (s1 <= s2 && e1 >= e2))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|((s1, e1), (s2, e2))| !((e1 < s2) || (e2 < s1)))
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
        assert_eq!(parse_to_assignment("2-4"), (2, 4))
    }
    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("1-3,2-4"), ((1, 3), (2, 4)))
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
