use itertools::Itertools;
use std::{collections::HashMap, fs};

fn solve(input: &str) -> String {
    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();

    // Populate crate information
    for line in input.lines().filter(|l| l.contains('[')) {
        println!("{}", line);
        for (index, item) in line.char_indices().filter(|(_, c)| ('A'..='Z').contains(c)) {
            let stack_index = (index / 4) + 1;
            let stack = crates.entry(stack_index).or_default();
            stack.insert(0, item);
        }
    }

    // Move boxes
    for instruction in input.lines().filter(|l| l.contains("move")) {
        let (quantity, from, to) = instruction
            .split(' ')
            .filter_map(|word| word.parse().ok())
            .next_tuple()
            .unwrap();

        let stack = crates.get_mut(&from).unwrap();
        let mut items = stack.split_off(stack.len() - quantity);
        crates.entry(to).or_default().append(&mut items);
    }

    // Return top box from each stack as string
    crates
        .keys()
        .sorted()
        .map(|k| {
            crates
                .get(k)
                .expect("stack doesn't exist")
                .last()
                .expect("each stack should have atleast one box")
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input/day-5.txt").unwrap();
    let answer = solve(&input);
    println!("The answer is: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("input/day-5-example.txt").unwrap();
        assert_eq!(solve(&input), "MCD");
    }
}
