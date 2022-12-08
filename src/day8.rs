use std::ops::Range;

#[derive(Debug, PartialEq)]
struct Tree {
    x: usize,
    y: usize,
    height: u32,
}

impl Tree {
    fn is_blocked_dir(&self, map: &Map, range: Range<usize>, vert: bool) -> bool {
        range.into_iter().any(|i| {
            if vert {
                map[i][self.x].height >= self.height
            } else {
                map[self.y][i].height >= self.height
            }
        })
    }
    fn score_dir(&self, map: &Map, iter: impl Iterator<Item = usize>, vert: bool) -> usize {
        let mut score = 0;
        for i in iter {
            let cond = if vert {
                map[i][self.x].height >= self.height
            } else {
                map[self.y][i].height >= self.height
            };
            score += 1;
            if cond {
                break;
            }
        }
        score
    }
    fn new(x: usize, y: usize, height: u32) -> Self {
        Self { x, y, height }
    }
    fn check_visible(&self, map: &Map) -> bool {
        let n_blocked = self.is_blocked_dir(map, 0..self.y, true);
        let s_blocked = self.is_blocked_dir(map, self.y + 1..map.len(), true);
        let e_blocked = self.is_blocked_dir(map, self.x + 1..map[0].len(), false);
        let w_blocked = self.is_blocked_dir(map, 0..self.x, false);
        !n_blocked || !s_blocked || !e_blocked || !w_blocked
    }
    fn calc_score(&self, map: &Map) -> usize {
        let m_height = map.len();
        let m_width = map[0].len();
        let ns = self.score_dir(map, (0..self.y).rev(), true);
        let ss = self.score_dir(map, self.y + 1..m_height, true);
        let es = self.score_dir(map, self.x + 1..m_width, false);
        let ws = self.score_dir(map, (0..self.x).rev(), false);
        ns * ss * es * ws
    }
}

type Map = Vec<Vec<Tree>>;

fn create_map(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .map(|(y_idx, l)| {
            l.chars()
                .enumerate()
                .map(|(x_idx, c)| Tree::new(x_idx, y_idx, c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let map = create_map(input);
    map.iter()
        .flatten()
        .filter(|t| t.check_visible(&map))
        .count()
}

fn part2(input: &str) -> usize {
    let map = create_map(input);
    map.iter()
        .flatten()
        .map(|t| t.calc_score(&map))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../input/day8.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day8.txt");
    #[test]
    fn check_visible_test() {
        let map = create_map(EXAMPLE);
        let tree = &map[1][1];
        assert!(tree.check_visible(&map));
        let tree = &map[1][2];
        assert!(tree.check_visible(&map));
        let tree = &map[2][1];
        assert!(tree.check_visible(&map));
        let tree = &map[2][3];
        assert!(tree.check_visible(&map));
    }
    #[test]
    fn check_not_vis() {
        let map = create_map(EXAMPLE);
        let tree = &map[2][2];
        assert!(!tree.check_visible(&map));
    }
    #[test]
    fn check_score() {
        let map = create_map(EXAMPLE);
        let tree = &map[1][2];
        assert_eq!(tree.calc_score(&map), 4);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 21);
    }
}
