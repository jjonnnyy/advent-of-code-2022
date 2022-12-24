use std::{cell::{RefCell, Cell}, collections::VecDeque, fs};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1, newline, one_of},
    combinator::{map_opt, map_res},
    multi::separated_list0,
    sequence::{delimited, pair, preceded},
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
    inspections: Cell<u32>,
    items: RefCell<VecDeque<u32>>,
    operation: Operation,
    divisor: u32,
    throw_true: usize,
    throw_false: usize,
}

fn monkey_id(input: &str) -> IResult<&str, u32> {
    let parser = delimited(tag("Monkey "), digit1, char(':'));
    let (input, id) = map_res(parser, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = newline(input)?;
    Ok((input, id))
}

fn starting_items(input: &str) -> IResult<&str, VecDeque<u32>> {
    let (input, _) = preceded(multispace1, tag("Starting items:"))(input)?;
    let item_parser = map_res(preceded(multispace0, digit1), |s: &str| s.parse::<u32>());
    let (input, items) = separated_list0(char(','), item_parser)(input)?;
    let (input, _) = newline(input)?;
    Ok((input, items.into()))
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

fn test(input: &str) -> IResult<&str, (u32, usize, usize)> {
    let (input, divisor) = map_res(
        delimited(
            preceded(multispace1, tag("Test: divisible by ")),
            digit1,
            newline,
        ),
        |s: &str| s.parse::<u32>(),
    )(input)?;
    let (input, true_monkey) = map_res(
        delimited(
            preceded(multispace1, tag("If true: throw to monkey ")),
            digit1,
            newline,
        ),
        |s: &str| s.parse::<usize>(),
    )(input)?;
    let (input, false_monkey) = map_res(
        delimited(
            preceded(multispace1, tag("If false: throw to monkey ")),
            digit1,
            newline,
        ),
        |s: &str| s.parse::<usize>(),
    )(input)?;
    Ok((input, (divisor, true_monkey, false_monkey)))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = monkey_id(input)?;
    let (input, items) = starting_items(input)?;
    let (input, operation) = operation(input)?;
    let (input, (divisor, throw_true, throw_false)) = test(input)?;

    Ok((
        input,
        Monkey {
            inspections: Cell::new(0),
            items: RefCell::new(items),
            operation,
            divisor,
            throw_true,
            throw_false,
        },
    ))
}

fn monkey_list(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(newline, monkey)(input)
}

fn part_one(input: &str) -> u32 {
    let (_, monkeys) = monkey_list(input).unwrap();

    for _ in 0..20 {
        for monkey in monkeys.iter() {
            let mut items = monkey.items.take();
            let inspection_count = monkey.inspections.take() + items.len() as u32;
            monkey.inspections.set(inspection_count);

            while let Some(item) = items.pop_front() {
                let worry_level = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                } / 3;

                if worry_level % monkey.divisor == 0 {
                    monkeys
                        .get(monkey.throw_true)
                        .unwrap()
                        .items
                        .borrow_mut()
                        .push_back(worry_level);
                } else {
                    monkeys
                        .get(monkey.throw_false)
                        .unwrap()
                        .items
                        .borrow_mut()
                        .push_back(worry_level);
                }
            }
        }
    }

    let inspection_counts = monkeys.into_iter().map(|m| m.inspections.take()).sorted().rev();
    inspection_counts.take(2).product()
}

fn part_two(_input: &str) -> u32 {
    0
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
