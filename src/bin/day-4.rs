use std::fs;

use itertools::Itertools;

fn first_part(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut components = line.split(&['-', ',']).map(|s| s.parse::<u32>().unwrap());
            let first_start = components.next().unwrap();
            let first_stop = components.next().unwrap();
            let second_start = components.next().unwrap();
            let second_stop = components.next().unwrap();

            let first_within_second = first_start >= second_start && first_stop <= second_stop;
            let second_within_first = second_start >= first_start && second_stop <= first_stop;
            first_within_second || second_within_first
        })
        .count()
}

fn second_part(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a_start, a_stop, b_start, b_stop) = line
                .split(&['-', ','])
                .map(|s| s.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap();

            let mut a_range = a_start..=a_stop;
            let b_range = b_start..=b_stop;

            a_range.any(|x| b_range.contains(&x))
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("input/day-4.txt").unwrap();
    println!("First part answer: {}", first_part(&input));
    println!("Second part answer: {}", second_part(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = fs::read_to_string("input/day-4-example.txt").unwrap();
        let result = first_part(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn second_example() {
        let input = fs::read_to_string("input/day-4-example.txt").unwrap();
        let result = second_part(&input);
        assert_eq!(result, 4);
    }
}
