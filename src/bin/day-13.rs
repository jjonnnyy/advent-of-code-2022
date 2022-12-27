use std::fs;

use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
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

fn part_one(input: &str) -> u32 {
    let (_, packet_pairs) = separated_list0(newline, packet_pair)(input).unwrap();
    0
}

fn part_two(_input: &str) -> u32 {
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
