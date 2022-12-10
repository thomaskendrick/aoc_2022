#[derive(Debug)]
enum OpCode {
    Add(i32),
    NoOp,
}

type SpriteMask = [bool; 40];
fn set_sprite_location(sm : &mut SpriteMask, loc: i32) {
    for (i, p) in sm.iter_mut().enumerate() {
        *p = loc >= i as i32 - 1 && loc <= i as i32 + 1
    }
}

struct Instruction(u8, OpCode);
impl Instruction {
    fn new (s: &str) -> Self {
        if let Some((_, val)) = s.split_once(' ') {
            Self(2, OpCode::Add(val.parse().unwrap()))
        } else {
            Self(1, OpCode::NoOp)
        }
    }
}

fn process_input(input: &str) -> (i32, String) {
    let mut ops: Vec<Instruction> = input.lines().rev().map(Instruction::new).collect();
    let mut cycle: u32 = 1;
    let mut register: i32 = 1;
    let mut signal_strength = 0;
    let mut sprite_mask = [false; 40];
    let mut display_output = String::new();
    let Instruction(mut timer, mut opcode): Instruction = ops.pop().unwrap();
    while timer > 0 || !ops.is_empty() {
        if timer == 0 {
            if let OpCode::Add(x) = opcode {
                register += x;
            }
            if let Some(Instruction(new_timer, new_op)) = ops.pop() {
                timer = new_timer;
                opcode = new_op
            }
        }
        timer -= 1;
        set_sprite_location(&mut sprite_mask, register);
        display_output.push(if sprite_mask[((cycle - 1) % 40) as usize] {
            '🟩'
        } else {
            '⬛'
        });

        if ((cycle) % 40) == 0 {
            display_output.push('\n');
        }
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

fn part2(input: &str) -> String {
    let (_, output) = process_input(input);
    output
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
        let expected = "\
🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛🟩🟩⬛⬛
🟩🟩🟩⬛⬛⬛🟩🟩🟩⬛⬛⬛🟩🟩🟩⬛⬛⬛🟩🟩🟩⬛⬛⬛🟩🟩🟩⬛⬛⬛🟩🟩🟩⬛⬛⬛🟩🟩🟩⬛
🟩🟩🟩🟩⬛⬛⬛⬛🟩🟩🟩🟩⬛⬛⬛⬛🟩🟩🟩🟩⬛⬛⬛⬛🟩🟩🟩🟩⬛⬛⬛⬛🟩🟩🟩🟩⬛⬛⬛⬛
🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛
🟩🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛⬛🟩🟩🟩🟩
🟩🟩🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛
";
        assert_eq!(part2(EXAMPLE), expected);
    }
}

