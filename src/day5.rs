#[derive(Debug, PartialEq)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn new(s: &str) -> Self {
        let mut lines = s.lines().rev();
        let mut stacks = Vec::new();

        for _ in lines.next().unwrap().trim().split_whitespace() {
            stacks.push(Vec::new());
        }

        for line in lines {
            let chars: Vec<char> = line
                .chars()
                .collect();
            for (i, stack) in stacks.iter_mut().enumerate() {
                let target = 1 + (i * 4);
                if chars[target].is_alphabetic() {
                    stack.push(chars[target])
                }
            }
        }
        Stacks(stacks)
    }

    fn execute(&mut self, i: Instruction) {
        for _ in 0..i.number {
            let cargo = self.0[i.source].pop().unwrap();
            self.0[i.destination].push(cargo);
        }
    }
    fn execute_with_cratemover9001(&mut self, i: Instruction) {
        let idx = self.0[i.source].len() - i.number as usize;
        let mut pile = self.0[i.source].split_off(idx);
        self.0[i.destination].append(&mut pile)
    }
    fn read_top_cargo(&self) -> String {
        let mut message = String::new();
        for stack in &self.0 {
            message.push(*stack.last().unwrap());
        }
        message
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    number: u32,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        Instruction {
            number: parts.nth(1).unwrap().parse().unwrap(),
            source: parts.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            destination: parts.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        }
    }
}

fn part1(input: &str) -> String {
    let (picture, instruction_str) = input.split_once("\n\n").unwrap();
    let mut stacks = Stacks::new(picture);
    let instructions: Vec<Instruction> = instruction_str
        .lines()
        .map(|l| Instruction::new(l))
        .collect();
    for instruction in instructions {
        stacks.execute(instruction)
    }
    stacks.read_top_cargo()
}

fn part2(input: &str) -> String {
    let (picture, instruction_str) = input.split_once("\n\n").unwrap();
    let mut stacks = Stacks::new(picture);
    let instructions: Vec<Instruction> = instruction_str
        .lines()
        .map(|l| Instruction::new(l))
        .collect();
    for instruction in instructions {
        stacks.execute_with_cratemover9001(instruction)
    }
    stacks.read_top_cargo()
}

fn main() {
    let input = include_str!("../input/day5.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day5.txt");

    #[test]
    fn create_stacks() {
        let (picture, _) = EXAMPLE.split_once("\n\n").unwrap();
        let stacks = Stacks::new(picture);
        assert_eq!(
            stacks,
            Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']])
        );
    }

    #[test]
    fn create_instruction() {
        let (_, instructions) = EXAMPLE.split_once("\n\n").unwrap();
        let instruction = Instruction::new(instructions.lines().next().unwrap());
        assert_eq!(
            instruction,
            Instruction {
                number: 1,
                source: 1,
                destination: 0
            }
        );
    }
    #[test]
    fn execute_instruction() {
        let (picture, instructions) = EXAMPLE.split_once("\n\n").unwrap();
        let mut stacks = Stacks::new(picture);
        let instruction = Instruction::new(instructions.lines().next().unwrap());
        stacks.execute(instruction);
        assert_eq!(
            stacks,
            Stacks(vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']])
        );
    }
    #[test]
    fn read_top() {
        let (picture, _) = EXAMPLE.split_once("\n\n").unwrap();
        let stacks = Stacks::new(picture);
        assert_eq!(stacks.read_top_cargo(), String::from("NDP"),);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), String::from("CMZ"));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), String::from("MCD"));
    }
}
