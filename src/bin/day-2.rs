use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid outcome"),
        }
    }
}

impl From<&str> for Play {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Cannot covert to rock, paper, or sissors"),
        }
    }
}

impl Play {
    fn value(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    /// 0 for loss, 3 for draw, 6 for win + value of play
    fn score(player: Play, opponent: Play) -> u32 {
        let score_from_game = match (&player, &opponent) {
            (Play::Rock, Play::Rock) => 3,
            (Play::Rock, Play::Paper) => 0,
            (Play::Rock, Play::Scissors) => 6,
            (Play::Paper, Play::Rock) => 6,
            (Play::Paper, Play::Paper) => 3,
            (Play::Paper, Play::Scissors) => 0,
            (Play::Scissors, Play::Rock) => 0,
            (Play::Scissors, Play::Paper) => 6,
            (Play::Scissors, Play::Scissors) => 3,
        };
        score_from_game + player.value()
    }

    /// Determines the play required for the desired outcome
    fn play_needed(opponent: &Play, outcome: &Outcome) -> Play {
        match (opponent, outcome) {
            (Play::Rock, Outcome::Win) => Play::Paper,
            (Play::Rock, Outcome::Loss) => Play::Scissors,
            (Play::Rock, Outcome::Draw) => Play::Rock,
            (Play::Paper, Outcome::Win) => Play::Scissors,
            (Play::Paper, Outcome::Loss) => Play::Rock,
            (Play::Paper, Outcome::Draw) => Play::Paper,
            (Play::Scissors, Outcome::Win) => Play::Rock,
            (Play::Scissors, Outcome::Loss) => Play::Paper,
            (Play::Scissors, Outcome::Draw) => Play::Scissors,
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day-2.txt") {
        let mut total_score = 0;
        for line in lines.flatten() {
            let mut components = line.split(' ');
            let opponent: Play = components.next().unwrap().into();
            let outcome: Outcome = components.next().unwrap().into();
            let you = Play::play_needed(&opponent, &outcome);

            total_score += Play::score(you, opponent);
        }
        println!("Total score: {}", total_score);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
