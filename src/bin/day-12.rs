use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
enum Square {
    Start,
    End,
    Level(u32),
}

struct HeightMap {
    data: Vec<Vec<Square>>,
}

impl HeightMap {
    fn parse_input(input: &str) -> HeightMap {
        HeightMap {
            data: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            'S' => Square::Start,
                            'E' => Square::End,
                            c if ('a'..='z').contains(&c) => Square::Level(c as u32 - 'a' as u32),
                            _ => panic!("Unexpected square value"),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn find_start(&self) -> (usize, usize) {
        let row = self
            .data
            .iter()
            .position(|r| r.contains(&Square::Start))
            .unwrap();
        let col = self.data[row]
            .iter()
            .position(|s| s == &Square::Start)
            .unwrap();
        (row, col)
    }

    fn find_end(&self) -> (usize, usize) {
        let row = self
            .data
            .iter()
            .position(|r| r.contains(&Square::End))
            .unwrap();
        let col = self.data[row]
            .iter()
            .position(|s| s == &Square::End)
            .unwrap();
        (row, col)
    }

    fn find_a(&self) -> Vec<(usize, usize)> {
        let mut squares = Vec::new();
        for (row_index, row_data) in self.data.iter().enumerate() {
            for (col_index, square) in row_data.iter().enumerate() {
                if square == &Square::Level(0) {
                    squares.push((row_index, col_index));
                }
            }
        }
        squares
    }

    fn get_height(&self, row: usize, col: usize) -> Option<u32> {
        let target = self.data.get(row)?.get(col)?;
        match target {
            Square::Start => Some(0),
            Square::End => Some(26),
            Square::Level(x) => Some(x.to_owned()),
        }
    }
}

fn part_one(input: &str) -> u32 {
    let height_map = HeightMap::parse_input(input);

    let mut squares_reached = HashMap::new();

    let (start_col, start_row) = height_map.find_start();
    let mut to_process = vec![(start_col, start_row, 0)];

    while let Some((row, col, length)) = to_process.pop() {
        squares_reached.insert((row, col), length);
        let height = height_map.get_height(row, col).unwrap();

        let mut candidate_move = |(row, col)| {
            if let Some(target_height) = height_map.get_height(row, col) {
                if target_height <= height + 1 {
                    let prev_length = squares_reached.get(&(row, col)).unwrap_or(&u32::MAX);
                    if length + 1 < *prev_length {
                        to_process.push((row, col, length + 1));
                    }
                }
            }
        };

        // Up
        if row > 0 {
            candidate_move((row - 1, col));
        }
        // Down
        candidate_move((row + 1, col));
        // Left
        if col > 0 {
            candidate_move((row, col - 1));
        }
        // Right
        candidate_move((row, col + 1));
    }

    let (end_row, end_col) = height_map.find_end();
    squares_reached
        .get(&(end_row, end_col))
        .expect("Did not reach end square")
        .to_owned()
}

fn part_two(input: &str) -> u32 {
    let height_map = HeightMap::parse_input(input);

    let mut squares_reached = HashMap::new();

    let mut to_process: Vec<_> = height_map
        .find_a()
        .into_iter()
        .map(|(row, col)| (row, col, 0))
        .collect();

    while let Some((row, col, length)) = to_process.pop() {
        squares_reached.insert((row, col), length);
        let height = height_map.get_height(row, col).unwrap();

        let mut candidate_move = |(row, col)| {
            if let Some(target_height) = height_map.get_height(row, col) {
                if target_height <= height + 1 {
                    let prev_length = squares_reached.get(&(row, col)).unwrap_or(&u32::MAX);
                    if length + 1 < *prev_length {
                        to_process.push((row, col, length + 1));
                    }
                }
            }
        };

        // Up
        if row > 0 {
            candidate_move((row - 1, col));
        }
        // Down
        candidate_move((row + 1, col));
        // Left
        if col > 0 {
            candidate_move((row, col - 1));
        }
        // Right
        candidate_move((row, col + 1));
    }

    let (end_row, end_col) = height_map.find_end();
    squares_reached
        .get(&(end_row, end_col))
        .expect("Did not reach end square")
        .to_owned()
}

fn main() {
    let input = fs::read_to_string("input/day-12.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-12-example.txt").unwrap();
        assert_eq!(part_one(&input), 31);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-12-example.txt").unwrap();
        assert_eq!(part_two(&input), 29);
    }
}
