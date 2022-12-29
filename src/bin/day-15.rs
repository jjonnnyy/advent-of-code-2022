use std::{collections::HashSet, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Sensor {
    location: (i64, i64),
    beacon: (i64, i64),
    distance: i64,
}

fn sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, x) = preceded(tag("Sensor at x="), i64)(input)?;
    let (input, y) = preceded(tag(", y="), i64)(input)?;
    let (input, beacon_x) = preceded(tag(": closest beacon is at x="), i64)(input)?;
    let (input, beacon_y) = preceded(tag(", y="), i64)(input)?;
    Ok((
        input,
        Sensor {
            location: (x, y),
            beacon: (beacon_x, beacon_y),
            distance: (x - beacon_x).abs() + (y - beacon_y).abs(),
        },
    ))
}

fn part_one(input: &str, row: i64) -> usize {
    let (_, sensors) = separated_list1(newline, sensor)(input).unwrap();
    let mut known_no_beacon = HashSet::new();

    for sensor in sensors.iter() {
        let dy = (row - sensor.location.1).abs();
        if dy < sensor.distance {
            let dx = sensor.distance - dy;
            for x in (sensor.location.0 - dx)..=(sensor.location.0 + dx) {
                known_no_beacon.insert((x, row));
            }
        }
    }

    for sensor in sensors {
        known_no_beacon.remove(&sensor.beacon);
    }

    known_no_beacon.iter().filter(|(_, y)| *y == row).count()
}

fn part_two(input: &str, max: i64) -> i64 {
    let (_, sensors) = separated_list1(newline, sensor)(input).unwrap();

    for x in 0..=max {
        if x % 1000 == 0 {
            dbg!(x);
        }
        for y in 0..=max {
            let within_range_of_sensor = sensors.iter().any(|sensor| {
                let distance_to_sensor =
                    (sensor.location.0 - x).abs() + (sensor.location.1 - y).abs();
                distance_to_sensor <= sensor.distance
            });
            if !within_range_of_sensor {
                return x * 4000000 + y;
            }
        }
    }
    panic!("Distress beacon not found");
}

fn main() {
    let input = fs::read_to_string("input/day-15.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input, 2000000));
    println!("Part two answer is: {}", part_two(&input, 4000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-15-example.txt").unwrap();
        assert_eq!(part_one(&input, 10), 26);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-15-example.txt").unwrap();
        assert_eq!(part_two(&input, 20), 56000011);
    }
}
