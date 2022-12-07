use std::collections::HashMap;

const REQUIRED_DISC_SPACE: usize = 30000000;

#[derive(Debug)]
enum Node {
    Directory(Vec<String>, HashMap<String, Node>),
    File(String, usize),
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Self::Directory(_, children) => children.values().map(|c| c.size()).sum(),
            Self::File(_, size) => *size,
        }
    }
    fn total_of_dir_with_max(&self, max: usize) -> usize {
        if let Self::Directory(_, children) = self {
            let size = self.size();
            let n = if size <= max { size } else { 0 };
            return n + children
                .values()
                .map(|c| c.total_of_dir_with_max(max))
                .sum::<usize>();
        }
        0
    }
}

fn part1(input: &str) -> usize {
    let root = build_fs(input);
    root.total_of_dir_with_max(100000)
}

fn part2(input: &str) -> usize {
    0
}

fn build_fs(input: &str) -> Node {
    let mut root = Node::Directory(vec!["/".to_string()], Default::default());
    let mut cwd: &mut Node = &mut root;

    for l in input.lines() {
        let (prefix, suffix) = l.split_once(" ").unwrap();
        match prefix {
            // A command
            "$" => {
                if suffix == "ls" {
                    continue;
                }
                let (_, suffix) = suffix.split_once(" ").unwrap();
                match suffix {
                    // Go to root
                    "/" => {
                        cwd = &mut root;
                    }
                    // Go to parent dir
                    ".." => {
                        let Node::Directory(path, _) = cwd else {panic!()};
                        let mut parent_directory: Vec<String> = path.iter().cloned().collect();
                        parent_directory.pop();

                        // Traverse the directory structure
                        for dir in parent_directory {
                            if dir == "/" {
                                cwd = &mut root;
                            } else {
                                let Node::Directory(path, children) = cwd else {panic!()};
                                cwd = children.get_mut(&dir).unwrap();
                            }
                        }
                    }
                    // If we're going into a new dir. Create it
                    dir => {
                        let Node::Directory(path, children) = cwd else {panic!()};
                        let mut new_path = path.clone();
                        new_path.push(dir.to_string());
                        cwd = children
                            .entry(dir.to_owned())
                            .or_insert(Node::Directory(new_path, Default::default()))
                    }
                }
            }
            // A file
            x if x.parse::<usize>().is_ok() => {
                let size: usize = x.parse().unwrap();
                let name = suffix.trim().to_owned();
                let Node::Directory(_cur, members) = cwd else { panic!() };
                let filenode = Node::File(name.clone(), size);
                members.insert(name, filenode);
            }
            _ => {}
        }
    }
    root
}

fn main() {
    let input = include_str!("../input/day7.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day7.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 95437);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
