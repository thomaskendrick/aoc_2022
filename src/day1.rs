fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let mut e: Vec<_> = input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect();
    e.sort();
    e[e.len() - 3..e.len()].iter().sum()
}

fn main() {
    let input = include_str!("../input/day1.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}
