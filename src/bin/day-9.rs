use std::{collections::HashSet, fs, str::FromStr};

enum RopeMove {
    Up,
    Down,
    Left,
    Right,
}

struct ParseRopeMoveError;

impl FromStr for RopeMove {
    type Err = ParseRopeMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(RopeMove::Up),
            "D" => Ok(RopeMove::Down),
            "L" => Ok(RopeMove::Left),
            "R" => Ok(RopeMove::Right),
            _ => Err(ParseRopeMoveError),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
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

    fn move_head(&mut self, direction: RopeMove) {
        todo!()
    }

    fn tail_visited_count(&self) -> usize {
        self.tail_visited.len()
    }
}

fn part_one(input: &str) -> usize {
    todo!()
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
