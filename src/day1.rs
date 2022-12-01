const INPUT: &str = include_str!("../input/day1.txt");

fn part1() -> i32 {
    INPUT
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2() -> i32 {
    let mut e: Vec<_> = INPUT
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect();
    e.sort();
    e[e.len() - 3..e.len()].iter().sum()
}

fn main() {
    aoc2022::solve_puzzles(part1, part2)
}
