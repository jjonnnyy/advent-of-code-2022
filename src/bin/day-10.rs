use core::fmt;
use std::{fs, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct ParseInstructionError(String);

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse instruction: {}", self.0)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() == 2 && parts[0] == "addx" {
            let v: i32 = parts[1]
                .parse()
                .map_err(|_| ParseInstructionError(String::from(s)))?;
            return Ok(Instruction::Addx(v));
        }

        Err(ParseInstructionError(String::from(s)))
    }
}

#[derive(Debug)]
struct Processor {
    x: i32,
    busy_remaining: u32,
    signal_strength: i32,
    current_instruction: Instruction,
}

impl Processor {
    fn new() -> Self {
        Processor {
            x: 1,
            busy_remaining: 0,
            signal_strength: 0,
            current_instruction: Instruction::Noop,
        }
    }

    fn signal_strength(&self) -> i32 {
        self.signal_strength
    }

    fn load_instruction(&mut self, instruction: Instruction) {
        self.busy_remaining = match instruction {
            Instruction::Noop => 0,
            Instruction::Addx(_) => 1,
        };
        self.current_instruction = instruction;
    }

    fn execute_current_instruction(&mut self) {
        match self.current_instruction {
            Instruction::Noop => {} // Do nothing
            Instruction::Addx(v) => self.x += v,
        }
        self.current_instruction = Instruction::Noop;
    }

    fn run(
        &mut self,
        sample_points: Vec<u32>,
        mut instructions: impl Iterator<Item = Instruction>,
    ) {
        let mut cycle_number = 0;
        let mut samples_remaining = sample_points.len();

        loop {
            if sample_points.contains(&cycle_number) {
                self.signal_strength += cycle_number as i32 * self.x;

                samples_remaining -= 1;
                if samples_remaining == 0 {
                    return;
                }
            }

            cycle_number += 1;

            if self.busy_remaining > 0 {
                self.busy_remaining -= 1;
                continue;
            }

            self.execute_current_instruction();

            if let Some(next_instruction) = instructions.next() {
                self.load_instruction(next_instruction);
            } else {
                return;
            }
        }
    }
}

fn part_one(input: &str) -> i32 {
    let mut processor = Processor::new();
    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap());
    let sample_points = vec![20, 60, 100, 140, 180, 220];

    processor.run(sample_points, instructions);
    processor.signal_strength()
}

fn part_two(_input: &str) -> i32 {
    0
}

fn main() {
    let input = fs::read_to_string("input/day-10.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-10-example.txt").unwrap();
        assert_eq!(part_one(&input), 13140);
    }

    #[test]
    #[ignore]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-10-example.txt").unwrap();
        assert_eq!(part_two(&input), 36);
    }
}
