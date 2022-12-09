use std::{collections::HashSet, fs};

fn part_one(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let width = lines[0].len();
    let height = lines.len();

    let trees: Vec<u32> = lines
        .into_iter()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap() + 1) // Plus one to count 0 height trees
        .collect();

    let mut visible_trees = HashSet::new();
    let calc_index = |x: usize, y: usize| -> usize { x + (y * width) };

    // From left
    for y in 0..height {
        let mut max = 0;
        for x in 0..width {
            let index = calc_index(x, y);
            if trees[index] > max {
                visible_trees.insert(index);
                max = trees[index];
            }
        }
    }

    // From right
    for y in 0..height {
        let mut max = 0;
        for x in (0..width).rev() {
            let index = calc_index(x, y);
            if trees[index] > max {
                visible_trees.insert(index);
                max = trees[index];
            }
        }
    }

    // From top
    for x in 0..width {
        let mut max = 0;
        for y in 0..height {
            let index = calc_index(x, y);
            if trees[index] > max {
                visible_trees.insert(index);
                max = trees[index];
            }
        }
    }

    // From below
    for x in 0..width {
        let mut max = 0;
        for y in (0..height).rev() {
            let index = calc_index(x, y);
            if trees[index] > max {
                visible_trees.insert(index);
                max = trees[index];
            }
        }
    }

    visible_trees.len() as u32
}

struct TreeData {
    trees: Vec<u32>,
    width: usize,
    height: usize,
}

fn score_tree(data: &TreeData, x: usize, y: usize) -> u32 {
    let calc_index = |x: usize, y: usize| -> usize { x + (y * data.width) };

    let height = data.trees[calc_index(x, y)];
    let mut score = 1;

    // to the left
    let mut count = 0;
    for check_x in (0..x).rev() {
        count += 1;
        let index = calc_index(check_x, y);
        if data.trees[index] >= height {
            break;
        }
    }
    score *= count;

    // to the right
    count = 0;
    for check_x in x + 1..data.width {
        count += 1;
        let index = calc_index(check_x, y);
        if data.trees[index] >= height {
            break;
        }
    }
    score *= count;

    // above
    count = 0;
    for check_y in (0..y).rev() {
        count += 1;
        let index = calc_index(x, check_y);
        if data.trees[index] >= height {
            break;
        }
    }
    score *= count;

    // below
    count = 0;
    for check_y in y + 1..data.height {
        count += 1;
        let index = calc_index(x, check_y);
        if data.trees[index] >= height {
            break;
        }
    }
    score *= count;

    score
}

fn part_two(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let width = lines[0].len();
    let height = lines.len();

    let trees: Vec<u32> = lines
        .into_iter()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let data = TreeData {
        trees,
        width,
        height,
    };

    let mut max_score = 0;
    for x in 0..data.width {
        for y in 0..data.height {
            let score = score_tree(&data, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn main() {
    let input = fs::read_to_string("input/day-8.txt").unwrap();
    println!("First part answer is {}", part_one(&input));
    println!("Second part answer is {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-8-example.txt").unwrap();
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-8-example.txt").unwrap();
        assert_eq!(part_two(&input), 8);
    }
}
