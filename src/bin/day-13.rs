use std::fs;

use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
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
            std::cmp::Ordering::Less => Comparison::Ordered,
            std::cmp::Ordering::Equal => Comparison::Same,
            std::cmp::Ordering::Greater => Comparison::Unordered,
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
                std::cmp::Ordering::Less => Comparison::Ordered,
                std::cmp::Ordering::Equal => Comparison::Same,
                std::cmp::Ordering::Greater => Comparison::Unordered,
            }
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

fn part_two(_input: &str) -> usize {
    0
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
    #[ignore]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-13-example.txt").unwrap();
        assert_eq!(part_two(&input), 29);
    }
}
