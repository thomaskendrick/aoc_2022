use std::collections::HashSet;

type Knot = (i32, i32);
type VisitSet = HashSet<Knot>;

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
    visits: VisitSet,
}
fn calc_tail_pos(h: &Knot, t: &Knot) -> Knot {
    let (h_x, h_y) = h;
    let (t_x, t_y) = t;
    let d_x = h_x - t_x;
    let d_y = h_y - t_y;
    if d_x.abs() > 1 {
        if d_y != 0 {
            return (t_x + d_x.signum(), t_y + d_y.signum());
        } else {
            return (t_x + d_x.signum(), *t_y);
        }
    }
    if d_y.abs() > 1 {
        if d_x != 0 {
            return (t_x + d_x.signum(), t_y + d_y.signum());
        } else {
            return (*t_x, t_y + d_y.signum());
        }
    }
    *t
}

impl Rope {
    fn new(init_knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); init_knots],
            visits: HashSet::new(),
        }
    }
    fn read_instruction(&mut self, s: &str) {
        let (dir, dist) = s.split_once(' ').unwrap();
        for _ in 0..dist.parse::<u32>().unwrap() {
            let (x, y) = self.knots[0];
            match dir {
                "U" => self.knots[0] = (x, y + 1),
                "D" => self.knots[0] = (x, y - 1),
                "L" => self.knots[0] = (x - 1, y),
                "R" => self.knots[0] = (x + 1, y),
                _ => panic!(),
            }
            for i in 0..self.knots.len() - 1 {
                self.knots[i + 1] = calc_tail_pos(&self.knots[i], &self.knots[i + 1])
            }
            self.visits.insert(*self.knots.last().unwrap());
        }
    }
}
fn part1(input: &str) -> usize {
    let mut rope = Rope::new(2);
    for line in input.lines() {
        rope.read_instruction(line);
    }
    rope.visits.len()
}

fn part2(input: &str) -> usize {
    let mut rope = Rope::new(10);
    for line in input.lines() {
        rope.read_instruction(line);
    }
    rope.visits.len()
}

fn main() {
    let input = include_str!("../input/day9.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day9.txt");
    const EXAMPLE_2: &str = include_str!("../example/day9_2.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 1);
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(part2(EXAMPLE_2), 36);
    }
}
