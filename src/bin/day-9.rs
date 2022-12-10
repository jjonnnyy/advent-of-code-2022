use std::{collections::HashSet, fs, str::FromStr};

use itertools::Itertools;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

struct Rope {
    head: Position,
    tail: Position,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
            tail_visited: HashSet::new(),
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        self.head.move_in_direction(direction);
        todo!()
    }

    fn tail_visited_count(&self) -> usize {
        self.tail_visited.len()
    }
}

fn part_one(input: &str) -> usize {
    let mut rope = Rope::new();

    for line in input.lines() {
        let (direction, count) = line.split(' ').next_tuple().unwrap();
        let direction = direction.parse::<Direction>().unwrap();
        let count = count.parse::<u8>().unwrap();

        for _ in 0..count {
            rope.move_head(&direction);
        }
    }

    rope.tail_visited_count()
}

fn main() {
    let input = fs::read_to_string("input/day-9.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-9-example.txt").unwrap();
        assert_eq!(part_one(&input), 13);
    }
}
