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
        },
    ))
}

fn part_one(input: &str, row: i64) -> usize {
    let (_, sensors) = separated_list1(newline, sensor)(input).unwrap();

    let mut known_no_beacon = HashSet::new();

    for sensor in sensors.iter() {
        let beacon_dx = (sensor.beacon.0 - sensor.location.0).abs();
        let beacon_dy = (sensor.beacon.1 - sensor.location.1).abs();
        let distance_to_beacon = f32::sqrt((beacon_dx * beacon_dx + beacon_dy * beacon_dy) as f32);

        let dy = (row - sensor.location.1).abs();
        if dy as f32 <= distance_to_beacon {
            for x in (sensor.location.0 - distance_to_beacon as i64)
                ..=(sensor.location.0 + distance_to_beacon as i64)
            {
                let dx = (x - sensor.location.0).abs();
                let distance_to_sensor = f32::sqrt((dx * dx + dy * dy) as f32);

                if distance_to_sensor <= distance_to_beacon {
                    known_no_beacon.insert((x, row));
                }
            }
        }
    }

    for sensor in sensors {
        known_no_beacon.remove(&sensor.beacon);
    }

    known_no_beacon.iter().filter(|(_, y)| *y == row).count()
}

fn part_two(_input: &str) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("input/day-15.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input, 2000000));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/day-15-example.txt").unwrap();
        assert_eq!(part_one(&input, 10), 26);
    }

    #[ignore]
    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/day-15-example.txt").unwrap();
        assert_eq!(part_two(&input), 0);
    }
}
