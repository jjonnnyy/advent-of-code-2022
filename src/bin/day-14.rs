use std::{collections::HashMap, fmt::Display, fs, hash::Hash};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coord(usize, usize);

fn coord(input: &str) -> IResult<&str, Coord> {
    let (input, x) = map_res(digit1, |d: &str| d.parse())(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y) = map_res(digit1, |d: &str| d.parse())(input)?;
    Ok((input, Coord(x, y)))
}

fn path(input: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(tag(" -> "), coord)(input)
}

#[derive(PartialEq)]
enum Material {
    Rock,
    Air,
    Sand,
}

impl Default for &Material {
    fn default() -> Self {
        &Material::Air
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Rock => write!(f, "#")?,
            Material::Air => write!(f, " ")?,
            Material::Sand => write!(f, "o")?,
        };
        Ok(())
    }
}

struct RockFace {
    data: HashMap<Coord, Material>,
    floor: usize,
}

impl RockFace {
    fn new(paths: &Vec<Vec<Coord>>) -> Self {
        let mut data = HashMap::new();

        for path in paths {
            for (start, stop) in path.iter().tuple_windows() {
                let x_range = if start.0 < stop.0 {
                    start.0..=stop.0
                } else {
                    stop.0..=start.0
                };

                let y_range = if start.1 < stop.1 {
                    start.1..=stop.1
                } else {
                    stop.1..=start.1
                };

                for x in x_range {
                    for y in y_range.clone() {
                        data.insert(Coord(x, y), Material::Rock);
                    }
                }
            }
        }

        Self {
            data,
            floor: paths.iter().flatten().map(|c| c.1).max().unwrap(),
        }
    }

    fn drop_sand(&mut self) -> Option<Coord> {
        let mut x = 500;
        for y in 0..=self.floor {
            let below = self.data.get(&Coord(x, y + 1)).unwrap_or_default();
            if *below == Material::Air {
                continue;
            }

            let left = self.data.get(&Coord(x - 1, y + 1)).unwrap_or_default();
            if *left == Material::Air {
                x -= 1;
                continue;
            }

            let right = self.data.get(&Coord(x + 1, y + 1)).unwrap_or_default();
            if *right == Material::Air {
                x += 1;
                continue;
            }

            self.data.insert(Coord(x, y), Material::Sand);
            return Some(Coord(x, y));
        }
        None
    }

    fn print(&self) {
        let mut keys_x = self.data.keys().map(|k| k.0).sorted();
        let mut keys_y = self.data.keys().map(|k| k.1).sorted();

        let min_x = keys_x.next().unwrap();
        let max_x = keys_x.last().unwrap();

        let min_y = keys_y.next().unwrap();
        let max_y = keys_y.last().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", self.data.get(&Coord(x, y)).unwrap_or(&Material::Air))
            }
            println!();
        }
        println!();
    }
}

fn part_one(input: &str) -> usize {
    let (_, paths) = separated_list1(newline, path)(input).unwrap();
    let mut rock_face = RockFace::new(&paths);
    rock_face.print();

    let mut count = 0;
    while rock_face.drop_sand().is_some() {
        count += 1;
    }

    rock_face.print();

    count
}

fn part_two(_input: &str) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("input/day-14.txt").unwrap();
    println!("Part one answer is: {}", part_one(&input));
    println!("Part two answer is: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE), 24);
    }

    #[test]
    #[ignore]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE), 140);
    }
}
