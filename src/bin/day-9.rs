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

#[derive(Clone, Hash, PartialEq, Eq, Default, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    /// Imprecise distance calculation
    fn distance(&self, other: &Position) -> u32 {
        let mut dx = other.x - self.x;
        let mut dy = other.y - self.y;
        if dx < 0 {
            dx *= -1;
        }
        if dy < 0 {
            dy *= -1;
        }

        if dx > dy {
            dx as u32
        } else {
            dy as u32
        }
    }

    fn move_by_one(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn follow(&mut self, other: &Position) {
        let distance = self.distance(other);

        if distance < 2 {
            return;
        }

        if distance > 2 {
            panic!("Expected point being followed to only move by one");
        }

        if self.x != other.x {
            if other.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }

        if self.y != other.y {
            if other.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
    }
}

struct Rope {
    knots: Vec<Position>,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn new(length: usize) -> Rope {
        let mut knots = Vec::new();
        for _ in 0..length {
            knots.push(Position::default());
        }

        Rope {
            knots,
            tail_visited: HashSet::new(),
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        self.knots[0].move_by_one(direction);

        for i in 1..self.knots.len() {
            let prev_knot = self.knots[i - 1].clone();
            self.knots[i].follow(&prev_knot);
        }

        let tail_position = self.knots[self.knots.len() - 1].clone();
        self.tail_visited.insert(tail_position);
    }

    fn tail_visited_count(&self) -> usize {
        self.tail_visited.len()
    }
}

fn perform_moves(input: &str, rope: &mut Rope) {
    for line in input.lines() {
        let (direction, count) = line.split(' ').next_tuple().unwrap();
        let direction = direction.parse::<Direction>().unwrap();
        let count = count.parse::<u8>().unwrap();

        for _ in 0..count {
            rope.move_head(&direction);
        }
    }
}

fn part_one(input: &str) -> usize {
    let mut rope = Rope::new(2);
    perform_moves(input, &mut rope);
    rope.tail_visited_count()
}

fn part_two(input: &str) -> usize {
    let mut rope = Rope::new(10);
    perform_moves(input, &mut rope);
    rope.tail_visited_count()
}

fn main() {
    let input = fs::read_to_string("input/day-9.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-9-example.txt").unwrap();
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-9-example1.txt").unwrap();
        assert_eq!(part_two(&input), 36);
    }
}
