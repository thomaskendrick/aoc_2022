use std::fmt::{Display, Debug};
use std::time::Instant;

pub fn solve_puzzles<T: Display + Debug>(
    input: &str,
    part1: impl FnOnce(&str) -> T,
    part2: impl FnOnce(&str) -> T,
) {

    let timer = Instant::now();
    let pt1_result = part1(input);
    let time = timer.elapsed();
    println!("------------------------\n");
    println!("Part 1 Answer: {pt1_result}");
    println!("Time taken: {time:.2?}\n");
    let timer = Instant::now();
    let pt2_result = part2(input);
    let time = timer.elapsed();
    println!("Part 2 Answer: {pt2_result}");
    println!("Time taken: {time:.2?}\n");
    println!("------------------------");
}
