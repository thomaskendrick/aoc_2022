use std::ops::Range;
use std::thread::sleep;
use std::time::Duration;
use kiss3d::camera::ArcBall;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, Point3};

#[derive(Debug, PartialEq)]
struct Tree {
    x: usize,
    y: usize,
    height: u32,
}

fn plot_map(map: &Map) {
    let mut window = Window::new("Kiss3d: cube");

    let mut nodes: Vec<_> = map.iter().flatten().map(|tree|
        {
            let mut c = window.add_cone(0.6, (tree.height as f32) + 1.0);
            c.set_local_translation(Translation3::new(tree.x as f32, (tree.height as f32 / 2.0) + 1.0, tree.y as f32));
            let mut trunk = window.add_cylinder(0.1, 0.5);
            trunk.set_color(0.58, 0.29, 0.0);
            trunk.set_local_translation(Translation3::new(tree.x as f32, 0.25, tree.y as f32));

            

            c.set_color(2.0 / (tree.calc_score(&map) + 1) as f32, 1.0 , 0.1);
            (tree, c)
        }
    ).collect();

    let mut camera = ArcBall::new_with_frustrum(90.0, 0.5, 200.0 , Point3::new(10.0, 10.0, 10.0), Point3::new(50.0, 0.0, 50.0));

    window.set_light(Light::Absolute(Point3::new(0.0, 0.0, 0.0)));
    let mut show_visible = false;
    while window.render_with_camera(&mut camera) {
        if let kiss3d::event::Action::Press = window.get_key(kiss3d::event::Key::F1) {
            show_visible = !show_visible;
            if show_visible {
                nodes.iter_mut().for_each(|(tree, c)| c.set_visible(tree.check_visible(&map)));
            } else {
                nodes.iter_mut().for_each(|(_, c)| c.set_visible(true));
            }
            sleep(Duration::from_millis(100))
        }
    }
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
        let ns = self.score_dir(map, (0..self.y).rev(), true);
        let ss = self.score_dir(map, self.y + 1..map.len(), true);
        let es = self.score_dir(map, self.x + 1..map[0].len(), false);
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
    let map = create_map(input);
    plot_map(&map);
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
