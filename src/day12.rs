use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;
use std::{error::Error, str::FromStr};
type Point = (i32, i32);

fn can_climb(start: u32, finish: u32) -> bool {
    finish - 1 <= start
}

#[derive(Debug)]
struct HeightMap {
    heights: HashMap<Point, u32>,
    start: Point,
    end: Point,
}

impl HeightMap {
    fn valid_moves(&self, (x, y): Point) -> Vec<Point> {
        let mut valid = Vec::new();
        let current = self.heights.get(&(x, y)).unwrap();
        let translations = [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)];
        for translation in translations {
            if let Some(c) = self.heights.get(&translation) {
                if can_climb(*current, *c) {
                    valid.push(translation);
                }
            }
        }
        valid
    }
}

impl FromStr for HeightMap {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;
        let heights: HashMap<Point, u32> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some((x as i32, y as i32));
                            ((x as i32, y as i32), 'a' as u32 - 96)
                        }
                        'E' => {
                            end = Some((x as i32, y as i32));
                            ((x as i32, y as i32), 'z' as u32 - 96)
                        }
                        c => ((x as i32, y as i32), c as u32 - 96),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok(Self {
            heights,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    pos: Point,
    neighbors: Vec<Point>,
    g: Rc<RefCell<i32>>,
    f: Rc<RefCell<i32>>,
}

impl Node {
    fn new(pos: Point, map: &HeightMap, g: Rc<RefCell<i32>>, f: Rc<RefCell<i32>>) -> Self {
        Self {
            pos,
            neighbors: map.valid_moves(pos),
            g,
            f,
        }
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(map: &HeightMap, input: &str) -> Result<Vec<Point>, ()> {
    fn calc_f((x1, y1): Point, (x2, y2): Point) -> i32 {
        (x1 - x2).abs() + (y1 - y2).abs()
    }
    let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
    let mut g_score: HashMap<Point, Rc<RefCell<i32>>> = HashMap::new();
    let mut f_score: HashMap<Point, Rc<RefCell<i32>>> = HashMap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut shortest_path = vec![];
    map.heights.keys().for_each(|k| {
        g_score.insert(*k, Rc::new(RefCell::new(i32::MAX)));
    });
    let g_ref = Rc::new(RefCell::new(0));
    g_score.insert(map.start, g_ref.clone()).unwrap();
    open_set.push(Node::new(
        map.start,
        map,
        g_ref,
        Rc::new(RefCell::new(calc_f(map.start, map.end))),
    ));

    while let Some(current) = open_set.pop() {
        if current.pos == map.end {
            shortest_path.clear();
            let mut c = current.pos;
            shortest_path.push(c);
            while let Some(point) = came_from.get(&c) {
                shortest_path.push(*point);
                c = *point;
            }
            shortest_path.reverse();
        }
        for neighbor in current.neighbors {
            let tentative_g_score = *current.g.borrow() + 1;
            let g_score = g_score.get(&neighbor).unwrap();
            if tentative_g_score < *g_score.borrow() {
                came_from.insert(neighbor, current.pos);
                *g_score.borrow_mut() = tentative_g_score;

                let f_score = f_score
                    .entry(neighbor)
                    .or_insert_with(|| Rc::new(RefCell::new(i32::MAX)));
                *f_score.borrow_mut() = calc_f(neighbor, map.end);

                let node = Node::new(neighbor, map, g_score.clone(), f_score.clone());
                if !open_set.iter().any(|x| x.pos == node.pos) {
                    open_set.push(node);
                }
            }
        }
    }
    if !shortest_path.is_empty() {
        Ok(shortest_path)
    } else {
        Err(()) 
    }
}

fn part1(input: &str) -> i32 {
    let map = input.parse::<HeightMap>().unwrap();
    let sp = a_star(&map, input);
    sp.unwrap().len() as i32 - 1
}

fn part2(input: &str) -> i32 {
    let mut map = input.parse::<HeightMap>().unwrap();
    let possible_start_points: Vec<_> = map
        .heights
        .iter()
        .filter_map(|(k, v)| if *v == 1 { Some(k.to_owned()) } else { None })
        .collect();
    possible_start_points
        .iter()
        .filter_map(|p| {
            map.start = *p;
            if let Ok(sp) =a_star(&map, input) {
                Some(sp.len() - 1)
            } else {
                None
            }
        })
        .min()
        .unwrap() as i32
}

fn main() {
    let input = include_str!("../input/day12.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day12.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 31);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 29);
    }
}
