advent_of_code::solution!(8);
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Antenna(char),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | '#' => Tile::Empty,
            c => Tile::Antenna(c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Antenna(c) => write!(f, "{}", c),
        }
    }
}

fn parse(input: &str) -> (HashMap<char, Vec<SignedPoint>>, SignedPoint) {
    let mut map: HashMap<_, Vec<_>> = HashMap::new();
    let mut max = SignedPoint::new(0, 0);
    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            max.x = line.len() as i64;
        }
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                map.entry(c).or_default().push(Point::new(x, y).cast());
            }
        }
        max.y += 1;
    }

    (map, max)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut antinodes = HashSet::new();

    let (antennas, max) = parse(input);
    let min = SignedPoint::new(0, 0);

    for (a1, a2) in antennas
        .into_iter()
        .flat_map(|(_, v)| v.into_iter().tuple_combinations())
    {
        let delta = a2 - a1;
        for k in [2, -1] {
            let p = a1 + delta * k;
            if p.is_contained_by(&min, &max) {
                antinodes.insert(p);
            }
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut antinodes = HashSet::new();

    let (antennas, max) = parse(input);
    let min = SignedPoint::new(0, 0);

    for (a1, a2) in antennas
        .into_iter()
        .flat_map(|(_, v)| v.into_iter().tuple_combinations())
    {
        let delta = a2 - a1;
        for sign in [-1, 1] {
            for k in 0.. {
                let p = a1 + delta * k * sign;
                if p.is_contained_by(&min, &max) {
                    antinodes.insert(p);
                } else {
                    break;
                }
            }
        }
    }

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let s = r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."#;
        let result = part_one(s);
        assert_eq!(result, Some(2));
        let s = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
.........."#;
        let result = part_one(s);
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let s = r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
.........."#;
        let result = part_two(s);
        assert_eq!(result, Some(9));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
