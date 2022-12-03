fn char_to_priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        c as i32 - 38
    } else {
        c as i32 - 96
    }
}

fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, bp| {
        let (c1, c2) = bp.split_at(bp.len() / 2);
        acc + char_to_priority(c2.chars().find(|c| c1.contains(*c)).unwrap())
    })
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |acc, g| {
            acc + char_to_priority(
                g[2].chars()
                    .find(|c| g[0].contains(*c) && g[1].contains(*c))
                    .unwrap(),
            )
        })
}

fn main() {
    let input = include_str!("../input/day3.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day3.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 157);
    }
    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 70);
    }
}
