use std::fmt::{Display, Debug};
use std::time::Instant;

const ANSI_YELLOW_BOLD: &str = "\x1B[1;33m";
const ANSI_GREEN_BOLD: &str = "\x1B[1;32m";
const ANSI_WHITE_BOLD: &str = "\x1B[1;37m";
const ANSI_RESET: &str = "\x1b[0m";
const CHRISTMAS_CHEER: &str = "🎄⭐🎅🎄⭐🎅🎄⭐🎅🎄⭐🎅";

pub fn solve_puzzles<T: Display + Debug>(
    input: &str,
    part1: impl FnOnce(&str) -> T,
    part2: impl FnOnce(&str) -> T,
) {
    let timer = Instant::now();
    let pt1_result = part1(input);
    let time = timer.elapsed();
    println!("\n{CHRISTMAS_CHEER}");
    println!("{ANSI_WHITE_BOLD}Part 1 Answer: {ANSI_GREEN_BOLD}{pt1_result}{ANSI_RESET}");
    println!("Time taken: {ANSI_YELLOW_BOLD}{time:.2?}{ANSI_RESET}\n");
    let timer = Instant::now();
    let pt2_result = part2(input);
    let time = timer.elapsed();
    println!("{ANSI_WHITE_BOLD}Part 2 Answer: {ANSI_GREEN_BOLD}{pt2_result}{ANSI_RESET}");
    println!("Time taken: {ANSI_YELLOW_BOLD}{time:.2?}{ANSI_RESET}");
    println!("{CHRISTMAS_CHEER}\n");
}
