use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part_one(input: &str) -> Result<usize, &str> {
    for (index, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Ok(index + 4);
        }
    }
    Err("Unable to find start signal")
}

fn part_two(input: &str) -> Result<usize, &str> {
    let chars: Vec<char> = input.chars().collect();
    for (index, window) in chars.windows(14).enumerate() {
        if window.iter().all_unique() {
            return Ok(index + 14);
        }
    }
    Err("Unable to find start signal")
}

#[allow(unused)]
fn part_two_hashmap(input: &str) -> Result<usize, &str> {
    let chars: Vec<char> = input.chars().collect();
    for (index, window) in chars.windows(14).enumerate() {
        let mut counts = HashMap::<char, u8>::new();
        for c in window {
            let count = counts.entry(*c).or_default();
            *count += 1;
        }

        if counts.values().all(|v| *v == 1) {
            return Ok(index + 14);
        }
    }
    Err("Unable to find start signal")
}

#[allow(unused)]
fn part_two_hashset(input: &str) -> Result<usize, &str> {
    let chars: Vec<char> = input.chars().collect();
    for (index, window) in chars.windows(14).enumerate() {
        let mut set = HashSet::new();
        if window.iter().all(|c| set.insert(c)) {
            return Ok(index + 14);
        }
    }
    Err("Unable to find start signal")
}

fn main() {
    let input = fs::read_to_string("input/day-6.txt").unwrap();
    println!(
        "Part one start signal index is {}",
        part_one(&input).unwrap()
    );
    println!(
        "Part two start signal index is {}",
        part_two(&input).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Ok(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Ok(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Ok(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Ok(11));

        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Ok(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Ok(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Ok(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Ok(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Ok(26));
    }
}
