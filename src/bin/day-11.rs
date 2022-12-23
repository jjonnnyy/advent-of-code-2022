use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit0, digit1, multispace0, multispace1, newline, one_of},
    combinator::{map_opt, map_res},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    divisor: u32,
    throw_true: u32,
    throw_false: u32,
}

fn monkey_id(input: &str) -> IResult<&str, u32> {
    let parser = delimited(tag("Monkey "), digit1, char(':'));
    let (input, id) = map_res(parser, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = newline(input)?;
    Ok((input, id))
}

fn starting_items(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = preceded(multispace1, tag("Starting items:"))(input)?;
    let item_parser = map_res(preceded(multispace0, digit1), |s: &str| s.parse::<u32>());
    let (input, items) = separated_list0(char(','), item_parser)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, items))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = preceded(multispace1, tag("Operation: new = old "))(input)?;
    let (input, operation) = map_opt(
        pair(
            one_of("*+"),
            preceded(multispace1, alt((tag("old"), digit1))),
        ),
        |(operator, value): (char, &str)| -> Option<Operation> {
            match (operator, value) {
                ('+', x) => {
                    let x = x.parse().ok()?;
                    Some(Operation::Add(x))
                }
                ('*', "old") => Some(Operation::Square),
                ('*', x) => {
                    let x = x.parse().ok()?;
                    Some(Operation::Multiply(x))
                }
                (_, _) => None,
            }
        },
    )(input)?;
    let (input, _) = newline(input)?;
    Ok((input, operation))
}

fn test(input: &str) -> IResult<&str, (u32, u32, u32)> {
    todo!()
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = monkey_id(input)?;
    let (input, items) = starting_items(input)?;
    let (input, operation) = operation(input)?;
    let (input, (divisor, throw_true, throw_false)) = test(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            divisor,
            throw_true,
            throw_false,
        },
    ))
}

fn parse_monkeys(input: &str) -> Option<Vec<Monkey>> {
    separated_list0(newline, monkey)(input).map(|(_, v)| v).ok()
}

fn part_one(input: &str) -> u32 {
    let monkeys = parse_monkeys(input).unwrap();
    dbg!(monkeys);
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
