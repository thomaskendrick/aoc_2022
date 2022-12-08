#[derive(Debug, PartialEq)]
struct Tree {
    x: usize,
    y: usize,
    height: u32,
}

impl Tree {
    fn new(x: usize, y: usize, height: u32) -> Self {
        Self { x, y, height }
    }
    fn check_visible(&self, map: &Map) -> bool {
        let m_height = map.len();
        let m_width = map[0].len();

        let mut is_vis = true;

        // Check north
        for i in 0..self.y {
            if map[i][self.x].height >= self.height {
                is_vis = false;
            }
        }
        if is_vis {
            return true;
        }

        // Check south
        is_vis = true;
        for i in self.y + 1..m_height {
            if map[i][self.x].height >= self.height {
                is_vis = false;
            }
        }
        if is_vis {
            return true;
        }

        // Check east
        is_vis = true;
        for i in self.x + 1..m_width {
            if map[self.y][i].height >= self.height {
                is_vis = false;
            }
        }
        if is_vis {
            return true;
        }

        // Check west
        is_vis = true;
        for i in 0..self.x {
            if map[self.y][i].height >= self.height {
                is_vis = false;
            }
        }
        is_vis
    }
    fn calc_score(&self, map: &Map) -> usize {
        let mut score = (0, 0, 0, 0);
        let m_height = map.len();
        let m_width = map[0].len();
        // Check north
        for i in (0..self.y).rev() {
            score.0 += 1;
            if map[i][self.x].height >= self.height {
                break;
            }
        }
        // Check south
        for i in self.y + 1..m_height {
            score.1 += 1;
            if map[i][self.x].height >= self.height {
                break;
            }
        }
        // Check east
        for i in self.x + 1..m_width {
            score.2 += 1;
            if map[self.y][i].height >= self.height {
                break;
            }
        }
        // Check west
        for i in (0..self.x).rev() {
            score.3 += 1;
            if map[self.y][i].height >= self.height {
                break;
            }
        }
        score.0 * score.1 * score.2 * score.3
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
