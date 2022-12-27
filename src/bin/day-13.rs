use std::{cmp::Ordering, fs};

use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Single(u32),
    List(Vec<Item>),
}

fn list_or_single(input: &str) -> IResult<&str, Item> {
    let (input, item) = alt((list, single))(input)?;
    Ok((input, item))
}

fn single(input: &str) -> IResult<&str, Item> {
    let (input, single_item) = map_res(digit1, |d: &str| d.parse())(input)?;
    Ok((input, Item::Single(single_item)))
}

fn list(input: &str) -> IResult<&str, Item> {
    let (input, list_items) = delimited(
        char('['),
        separated_list0(char(','), list_or_single),
        char(']'),
    )(input)?;
    Ok((input, Item::List(list_items)))
}

fn packet_pair(input: &str) -> IResult<&str, (Item, Item)> {
    let (input, first) = terminated(list, newline)(input)?;
    let (input, second) = terminated(list, newline)(input)?;
    Ok((input, (first, second)))
}

#[derive(Debug, PartialEq)]
enum Comparison {
    Ordered,
    Unordered,
    Same,
}

fn compare_items(left: &Item, right: &Item) -> Comparison {
    match (left, right) {
        (Item::Single(l), Item::Single(r)) => match l.cmp(r) {
            Ordering::Less => Comparison::Ordered,
            Ordering::Equal => Comparison::Same,
            Ordering::Greater => Comparison::Unordered,
        },
        (Item::Single(_), Item::List(_)) => compare_items(&Item::List(vec![left.clone()]), right),
        (Item::List(_), Item::Single(_)) => compare_items(left, &Item::List(vec![right.clone()])),
        (Item::List(l), Item::List(r)) => {
            for (l, r) in l.iter().zip(r) {
                match compare_items(l, r) {
                    Comparison::Ordered => return Comparison::Ordered,
                    Comparison::Unordered => return Comparison::Unordered,
                    Comparison::Same => continue,
                }
            }
            match l.len().cmp(&r.len()) {
                Ordering::Less => Comparison::Ordered,
                Ordering::Equal => Comparison::Same,
                Ordering::Greater => Comparison::Unordered,
            }
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare_items(self, other) {
            Comparison::Ordered => Ordering::Less,
            Comparison::Unordered => Ordering::Greater,
            Comparison::Same => Ordering::Equal,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match compare_items(self, other) {
            Comparison::Ordered => Some(Ordering::Less),
            Comparison::Unordered => Some(Ordering::Greater),
            Comparison::Same => Some(Ordering::Equal),
        }
    }
}

fn part_one(input: &str) -> usize {
    let (_, packet_pairs) = separated_list0(newline, packet_pair)(input).unwrap();

    let mut ordered_indexes = Vec::new();

    for (index, (left, right)) in packet_pairs.into_iter().enumerate() {
        if compare_items(&left, &right) != Comparison::Unordered {
            ordered_indexes.push(index + 1);
        }
    }

    ordered_indexes.into_iter().sum()
}

fn part_two(input: &str) -> usize {
    let (_, first_distress) = list("[[2]]").unwrap();
    let (_, second_distress) = list("[[6]]").unwrap();

    let mut packets: Vec<Item> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (_, list) = list(l).unwrap();
            list
        })
        .collect();

    packets.push(first_distress.clone());
    packets.push(second_distress.clone());
    packets.sort();

    let first = packets.iter().position(|i| *i == first_distress).unwrap() + 1;
    let second = packets.iter().position(|i| *i == second_distress).unwrap() + 1;
    first * second
}

fn main() {
    let input = fs::read_to_string("input/day-13.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-13-example.txt").unwrap();
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-13-example.txt").unwrap();
        assert_eq!(part_two(&input), 140);
    }
}
