use std::collections::HashMap;
use std::iter::once;

// Pt 1
const DIRECTORY_MAX_SIZE: usize = 100000;

// Pt 2
const TOTAL_DISC_SPACE: usize = 70000000;
const REQUIRED_DISC_SPACE: usize = 30000000;

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
    fn smallest_directory_to_free(&self, size_to_free: usize) -> Option<usize> {
        let size = self.size();
        if size <= size_to_free {
            None
        } else {
            if let Self::Directory(_p, children) = self {
                return children
                    .values()
                    .filter_map(|c| c.smallest_directory_to_free(size_to_free))
                    .chain(once(size))
                    .min();
            }
            None
        }
    }
}

fn build_fs(input: &str) -> Node {
    let mut root = Node::Directory(vec!["/".to_string()], Default::default());
    let mut cwd: &mut Node = &mut root;

    for l in input.lines() {
        let (prefix, suffix) = l.split_once(" ").unwrap();
        match prefix {
            // We're processing a command
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
                        if let Node::Directory(path, _) = cwd {
                            // Traverse the directory structure of the current file.
                            for dir in path[..path.len() - 1].to_vec() {
                                if dir == "/" {
                                    cwd = &mut root;
                                } else {
                                    if let Node::Directory(_, children) = cwd {
                                        cwd = children.get_mut(&dir).unwrap();
                                    }
                                }
                            }
                        }
                    }
                    // Create a new directory if we've not been there before.
                    dir => {
                        if let Node::Directory(path, children) = cwd {
                            let mut new_path = path.clone();
                            new_path.push(dir.to_string());
                            cwd = children
                                .entry(dir.to_string())
                                .or_insert(Node::Directory(new_path, Default::default()))
                        }
                    }
                }
            }
            // We're processing a file
            x if x.parse::<usize>().is_ok() => {
                let size: usize = x.parse().unwrap();
                let name = suffix.trim().to_owned();
                if let Node::Directory(_, children) = cwd {
                    let filenode = Node::File(name.clone(), size);
                    children.insert(name, filenode);
                }
            }
            // No-op for directory listings as we create them when navigated to.
            _ => {}
        }
    }
    root
}

fn part1(input: &str) -> usize {
    let root = build_fs(input);
    root.total_of_dir_with_max(DIRECTORY_MAX_SIZE)
}

fn part2(input: &str) -> usize {
    let root = build_fs(input);
    let required_space = REQUIRED_DISC_SPACE - (TOTAL_DISC_SPACE - root.size());
    root.smallest_directory_to_free(required_space).unwrap()
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
        assert_eq!(part2(EXAMPLE), 24933642);
    }
}
