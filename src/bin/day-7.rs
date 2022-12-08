use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

type DirRef = Rc<RefCell<Dir>>;

#[derive(Default, Debug)]
struct Dir {
    parent: Option<DirRef>,
    size_of_files: usize,
    sub_directories: HashMap<String, DirRef>,
}

fn parse_input(input: &str) -> DirRef {
    let root = Rc::new(RefCell::new(Dir::default()));
    let mut cursor = root.clone();

    let mut iter = input.lines().peekable();
    while let Some(line) = iter.next() {
        if line.starts_with("$ cd") {
            let dest = line.split(' ').last().unwrap();
            if dest == "/" {
                cursor = root.clone();
            } else if dest == ".." {
                let parent = cursor.borrow().parent.as_ref().unwrap().clone();
                cursor = parent;
            } else {
                let dest_dir = cursor.borrow().sub_directories.get(dest).unwrap().clone();
                cursor = dest_dir;
            }
            continue;
        }

        assert!(line.starts_with("$ ls"));

        while iter.peek().is_some() && !iter.peek().unwrap().starts_with('$') {
            let (dir_or_size, name) = iter.next().unwrap().split(' ').next_tuple().unwrap();

            if dir_or_size == "dir" {
                let new_dir = Rc::new(RefCell::new(Dir {
                    parent: Some(cursor.clone()),
                    size_of_files: 0,
                    sub_directories: HashMap::new(),
                }));
                (*cursor)
                    .borrow_mut()
                    .sub_directories
                    .insert(name.into(), new_dir);
            } else {
                (*cursor).borrow_mut().size_of_files += dir_or_size.parse::<usize>().unwrap();
            }
        }
    }
    root
}

fn get_sizes_recursive(dir: &DirRef, path: &str, sizes: &mut HashMap<String, usize>) -> usize {
    let current_dir = dir.borrow();

    if current_dir.sub_directories.is_empty() {
        return current_dir.size_of_files;
    }

    let mut total_size = current_dir.size_of_files;
    for (name, subdir) in current_dir.sub_directories.iter() {
        let sub_path = format!("{}/{}", path, name);
        let size = get_sizes_recursive(subdir, &sub_path, sizes);
        sizes.insert(sub_path, size);
        total_size += size;
    }
    total_size
}

fn get_sizes(root_dir: &DirRef) -> HashMap<String, usize> {
    let mut sizes = HashMap::new();
    let root_path = String::from("");
    let root_size = get_sizes_recursive(root_dir, &root_path, &mut sizes);
    sizes.insert(root_path, root_size);
    sizes
}

fn part_one(input: &str) -> usize {
    let root = parse_input(input);
    let sizes = get_sizes(&root);

    // find sum of directories with size of at most 100000
    sizes.into_values().filter(|v| *v <= 100000).sum()
}

fn part_two(input: &str) -> usize {
    let root = parse_input(input);
    let sizes = get_sizes(&root);

    // find sum of directories with size of at most 100000
    let total_used = sizes.get("").unwrap();
    let remaining = 70000000 - total_used;
    assert!(remaining < 30000000);
    let target_to_delete = 30000000 - remaining;

    sizes
        .into_values()
        .filter(|v| *v > target_to_delete)
        .sorted()
        .next()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input/day-7.txt").unwrap();
    println!("First part answer is {}", part_one(&input));
    println!("Second part answer is {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-7-example.txt").unwrap();
        assert_eq!(part_one(&input), 95437);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-7-example.txt").unwrap();
        assert_eq!(part_two(&input), 24933642);
    }
}
