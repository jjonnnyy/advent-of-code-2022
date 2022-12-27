use std::fs;

fn part_one(_input: &str) -> usize {
    0
}

fn part_two(_input: &str) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("input/day-14.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE), 24);
    }

    #[test]
    #[ignore]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE), 140);
    }
}
