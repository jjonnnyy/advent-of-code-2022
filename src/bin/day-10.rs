use std::fs;

fn part_one(input: &str) -> usize {
    todo!()
}

fn part_two(_input: &str) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("input/day-10.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-10-example.txt").unwrap();
        assert_eq!(part_one(&input), 13140);
    }

    #[test]
    #[ignore]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-10-example.txt").unwrap();
        assert_eq!(part_two(&input), 36);
    }
}
