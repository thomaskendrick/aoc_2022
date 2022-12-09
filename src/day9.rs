use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug)]
struct Rope {
    h_pos: Point,
    t_pos: Point,
    t_visit: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        Self {
            h_pos: (0, 0),
            t_pos: (0, 0),
            t_visit: HashSet::new(),
        }
    }

    fn read_instruction(&mut self, s: &str) {
        let (dir, dist) = s.split_once(' ').unwrap();
        for _ in 0..dist.parse::<u32>().unwrap() {
            let (x, y) = self.h_pos;
            match dir {
                "U" => self.h_pos = (x, y + 1),
                "D" => self.h_pos = (x, y - 1),
                "L" => self.h_pos = (x - 1, y),
                "R" => self.h_pos = (x + 1, y),
                _ => panic!(),
            }
            self.update_tail();
            self.t_visit.insert(self.t_pos);
        }
    }
    fn update_tail(&mut self) {
        let (h_x, h_y) = self.h_pos;
        let (t_x, t_y) = self.t_pos;
        let d_x = h_x - t_x;
        let d_y = h_y - t_y;
        if d_x.abs() > 1 {
            if d_y != 0 {
                self.t_pos = (t_x + d_x.signum(), t_y + d_y.signum())
            } else {
                self.t_pos = (t_x + d_x.signum(), t_y)
            }
        }
        if d_y.abs() > 1 {
            if d_x != 0 {
                self.t_pos = (t_x + d_x.signum(), t_y + d_y.signum())
            } else {
                self.t_pos = (t_x, t_y + d_y.signum())
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut rope = Rope::new();
    for line in input.lines() {
        rope.read_instruction(line)
    }
    rope.t_visit.len()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let input = include_str!("../input/day9.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day9.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
