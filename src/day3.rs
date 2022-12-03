fn char_to_priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        c as i32 - 38
    } else {
        c as i32 - 96
    }
}

fn part1(input: &str) -> i32 {
    let mut total = 0;
    'outer: for backpack in input.lines() {
        let (c1, c2) = backpack.split_at(backpack.len() / 2);
        for char in c2.chars() {
            if c1.contains(char) {
                total += char_to_priority(char);
                continue 'outer;
            }
        }
    }
    total
}

fn part2(input: &str) -> i32 {
    let mut total = 0;
    for g in input.lines().collect::<Vec<_>>().chunks(3) {
        let mut common: Vec<char> = Vec::new();
        for c in g[1].chars() {
            if g[0].contains(c) {
                common.push(c);
            }
        }
        for c in g[2].chars() {
            if common.contains(&c) {
                total += char_to_priority(c);
                break;
            }
        }
    }
    total
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
