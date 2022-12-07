use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut totals = BinaryHeap::new();
    let mut current_total = 0;

    for line in read_lines("./input/day-1.txt").unwrap().flatten() {
        if line.is_empty() {
            totals.push(current_total);
            current_total = 0;
            continue;
        }

        let value: u32 = line.parse().unwrap();
        current_total += value;
    }
    let mut sum = 0;
    for _ in 0..3 {
        if let Some(total) = totals.pop() {
            sum += total;
        }
    }
    println!("Sum of top 3 seen: {}", sum);
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
