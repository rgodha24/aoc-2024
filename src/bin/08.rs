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

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let mut antinodes = HashSet::new();
    let mut antennas: HashMap<char, Vec<_>> = HashMap::new();

    grid.flat_iter().for_each(|(tile, point)| match tile {
        Tile::Antenna(c) => antennas
            .entry(*c)
            .or_default()
            .push(point.as_signed_point()),
        _ => {}
    });

    for (&a1, &a2) in antennas
        .values()
        .flat_map(|v| v.iter().tuple_combinations())
    {
        let delta = a2 - a1;
        if grid.contains_point(a2 + delta) {
            antinodes.insert(a2 + delta);
        }
        if grid.contains_point(a1 - delta) {
            antinodes.insert(a1 - delta);
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let mut antinodes = HashSet::new();
    let mut antennas: HashMap<char, Vec<_>> = HashMap::new();

    grid.flat_iter().for_each(|(tile, point)| match tile {
        Tile::Antenna(c) => antennas
            .entry(*c)
            .or_default()
            .push(point.as_signed_point()),
        _ => {}
    });

    for (&a1, &a2) in antennas
        .values()
        .flat_map(|v| v.iter().tuple_combinations())
    {
        let delta = a2 - a1;
        for sign in [-1, 1] {
            for k in 0..100 {
                if grid.contains_point(a2 + delta * sign * k) {
                    antinodes.insert(a2 + delta * sign * k);
                }
                if grid.contains_point(a1 - delta * sign * k) {
                    antinodes.insert(a1 - delta * sign * k);
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
