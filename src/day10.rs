use std::{error::Error, str::FromStr};

#[derive(Debug)]
enum OpCode {
    Add(i32),
    NoOp,
}

#[derive(Debug)]
struct Sprite([bool; 40]);

impl Sprite {
    fn new() -> Self {
        Self([false; 40])
    }
    fn set(&mut self, loc: i32) {
        let Sprite(pixels) = self;
        for (i, p) in pixels.iter_mut().enumerate() {
            if loc == i as i32 || loc == i as i32 - 1 || loc == i as i32 + 1 {
                *p = true;
            } else {
                *p = false;
            }
        }
    }
}

struct Instruction(u8, OpCode);

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((_, val)) = s.split_once(' ') {
            Ok(Self(2, OpCode::Add(val.parse()?)))
        } else {
            Ok(Self(1, OpCode::NoOp))
        }
    }
}

fn process_input(input: &str) -> (i32, String) {
    let mut ops: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    ops.reverse();

    let mut cycle: u32 = 1;
    let mut register: i32 = 1;
    let mut signal_strength = 0;
    let mut sprite = Sprite::new();
    let mut display_output = String::new();
    let Instruction(mut timer, mut opcode): Instruction = ops.pop().unwrap();
    while timer > 0 || !ops.is_empty() {
        sprite.set(register);
        let Sprite(pixels) = sprite;

        if pixels[((cycle) % 40) as usize] {
            display_output.push('#');
        } else {
            display_output.push('.');
        }

        if (cycle % 40) == 0 {
            display_output.push('\n');
        }

        if timer == 0 {
            let new = ops.pop();
            if let OpCode::Add(x) = opcode {
                register += x;
            }
            if let Some(Instruction(new_timer, new_op)) = new {
                timer = new_timer;
                opcode = new_op
            }
        }
        timer -= 1;
        if ((cycle + 20) % 40) == 0 {
            signal_strength += cycle as i32 * register;
        }
        cycle += 1;
    }
    (signal_strength, display_output)
}

fn part1(input: &str) -> i32 {
    let (ss, _) = process_input(input);
    ss
}

fn part2(input: &str) -> i32 {
    let (_, output) = process_input(input);
    print!("{output}");
    0
}

fn main() {
    let input = include_str!("../input/day10.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day10.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 13140);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 1);
    }
}