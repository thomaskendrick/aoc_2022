use std::collections::VecDeque;
#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Op,
    test: usize,
    targets: (u8, u8),
    inspections: usize,
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
                    (x, y) if x == "+" => Op::Add(y.parse().unwrap()),
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

    fn yeet(&mut self, common_factor: Option<usize>) -> Option<(usize, u8)> {
        if let Some(item) = self.items.pop_front() {
            let mut item = match self.operation {
                Op::Square => item * item,
                Op::Add(x) => item + x,
                Op::Multiply(x) => item * x,
            };
            if let Some(common_factor) = common_factor {
                item %= common_factor;
            } else {
                item /= 3;
            }
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

trait CanPlay {
    fn play(&mut self, rounds: usize, calc_cf: bool) -> usize;
}

impl CanPlay for [Monkey] {
    fn play(&mut self, rounds: usize, calc_cf: bool) -> usize {
        let cf: Option<usize> = if calc_cf {
            Some(self.iter().map(|m| m.test).product())
        } else {
            None
        };
        for _ in 0..rounds {
            for i in 0..self.len() {
                while let Some((thrown_item, target)) = self[i].yeet(cf) {
                    self[target as usize].catch_item(thrown_item)
                }
            }
        }
        self.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));
        self[0].inspections * self[1].inspections
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Monkey::new)
        .collect::<Vec<Monkey>>()
        .play(20, false)
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Monkey::new)
        .collect::<Vec<Monkey>>()
        .play(10000, true)
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
        assert_eq!(part2(EXAMPLE), 2713310158);
    }
}
