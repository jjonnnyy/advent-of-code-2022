use std::fs;

fn part_one(input: &str) -> u32 {
    todo!()
}

fn part_two(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = fs::read_to_string("input/day-11.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-11-example.txt").unwrap();
        assert_eq!(part_one(&input), 10605);
    }
}
