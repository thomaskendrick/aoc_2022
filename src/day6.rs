use std::collections::HashSet;

fn detect_uniq_marker(input: &str, n: usize) -> i32 {
    let chars = input.chars().collect::<Vec<char>>();
    let (pos, _) = chars
        .windows(n)
        .enumerate()
        .find(|(_, window)| {
            let mut uniq: HashSet<char> = HashSet::new();
            window.iter().all(|c| uniq.insert(*c));
            uniq.len() == n
        })
        .unwrap();
    pos as i32 + n as i32
}

fn part1(input: &str) -> i32 {
    detect_uniq_marker(input, 4)
}

fn part2(input: &str) -> i32 {
    detect_uniq_marker(input, 14)
}

fn main() {
    let input = include_str!("../input/day6.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}
