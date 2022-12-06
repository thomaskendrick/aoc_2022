use std::collections::HashSet;

fn detect_uniq_marker(input: &str, n: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .position(|window| window.iter().copied().collect::<HashSet<char>>().len() == n)
        .unwrap()
        + n
}

fn part1(input: &str) -> usize {
    detect_uniq_marker(input, 4)
}

fn part2(input: &str) -> usize {
    detect_uniq_marker(input, 14)
}

fn main() {
    let input = include_str!("../input/day6.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}
