use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

trait Priority {
    /// If value has priority returns Some(priority), else None
    fn priority(self) -> Option<u32>;
}

impl Priority for char {
    fn priority(self) -> Option<u32> {
        match self {
            x if ('a'..='z').contains(&x) => Some(x as u32 - 'a' as u32 + 1),
            x if ('A'..='Z').contains(&x) => Some(x as u32 - 'A' as u32 + 27),
            _ => None,
        }
    }
}

#[test]
fn expected_priority() {
    // a = 1, z = 26, A = 27, Z = 52
    assert_eq!('a'.priority(), Some(1));
    assert_eq!('z'.priority(), Some(26));
    assert_eq!('A'.priority(), Some(27));
    assert_eq!('Z'.priority(), Some(52));
    assert_eq!('*'.priority(), None);
}

fn main() {
    let mut sum = 0;
    let mut iter = read_lines("./input/day-3.txt").unwrap().flatten();

    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        // Collect into set to deduplicate
        let first: HashSet<char> = first.chars().collect();

        for item in first {
            if second.contains(item) && third.contains(item) {
                sum += item.priority().unwrap();
            }
        }
    }

    println!("Sum of items in both: {}", sum);
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
