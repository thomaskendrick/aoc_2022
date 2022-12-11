use std::{collections::VecDeque};
#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Op,
    test: usize,
    targets: (u8, u8),
    inspections: u32,
}

#[derive(Debug)]
enum Op {
    Add(usize),
    Square,
    Multiply(usize),
}

impl Monkey {
    fn new(s: &str) -> Self {
        let mut spl = s.lines().skip(1);
        Self {
            items: spl
                .next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect(),
            operation: {
                match spl
                    .next()
                    .unwrap()
                    .split_once("old ")
                    .unwrap()
                    .1
                    .split_once(' ')
                    .unwrap()
                {
                    (x, y) if x == "+" => {
                        Op::Add(y.parse().unwrap())
                    }
                    (x, y) if x == "*" => {
                        if y == "old" {
                            Op::Square
                        } else {
                            Op::Multiply(y.parse().unwrap())
                        }
                    }
                    _ => panic!(),
                }
            },
            test: spl
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            targets: (
                spl.next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap(),
                spl.next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
            inspections: 0,
        }
    }

    fn yeet(&mut self) -> Option<(usize, u8)> {
        if let Some(item) = self.items.pop_front() {
            let mut item = match self.operation {
                Op::Square => item * item,
                Op::Add(x) => item + x,
                Op::Multiply(x) => item * x,
            };
            item /= 3;
            self.inspections += 1;
            let (t_target, f_target) = self.targets;
            if item % self.test == 0 {
                Some((item, t_target))
            } else {
                Some((item, f_target))
            }
        } else {
            None
        }
    }
    fn catch_item(&mut self, item: usize) {
        self.items.push_back(item);
    }
}
fn part1(input: &str) -> u32 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((thrown_item, target)) = monkeys[i].yeet() {
                monkeys[target as usize].catch_item(thrown_item)
            }
        }
    }
    monkeys.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

fn part2(input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("../input/day11.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day11.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 10605);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
