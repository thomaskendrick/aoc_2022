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
        let n_blocked = (0..self.y)
            .into_iter()
            .any(|i| map[i][self.x].height >= self.height);
        let s_blocked = (self.y + 1..m_height)
            .into_iter()
            .any(|i| map[i][self.x].height >= self.height);
        let e_blocked = (self.x + 1..m_width)
            .into_iter()
            .any(|i| map[self.y][i].height >= self.height);
        let w_blocked = (0..self.x)
            .into_iter()
            .any(|i| map[self.y][i].height >= self.height);
        !n_blocked || !s_blocked || !e_blocked || !w_blocked
    }
    fn calc_score(&self, map: &Map) -> usize {
        let (mut ns, mut ss, mut es, mut ws) = (0, 0, 0, 0);
        let m_height = map.len();
        let m_width = map[0].len();
        // Check north
        for i in (0..self.y).rev() {
            ns += 1;
            if map[i][self.x].height >= self.height {
                break;
            }
        }
        // Check south
        for i in self.y + 1..m_height {
            ss += 1;
            if map[i][self.x].height >= self.height {
                break;
            }
        }
        // Check east
        for i in self.x + 1..m_width {
            es += 1;
            if map[self.y][i].height >= self.height {
                break;
            }
        }
        // Check west
        for i in (0..self.x).rev() {
            ws += 1;
            if map[self.y][i].height >= self.height {
                break;
            }
        }
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
