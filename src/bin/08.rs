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
    Antinode,
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
            Tile::Antinode => write!(f, "#"),
            Tile::Antenna(c) => write!(f, "{}", c),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);
    let mut antinodes = HashSet::new();
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    grid.flat_iter().for_each(|(tile, point)| match tile {
        Tile::Antenna(c) => antennas.entry(*c).or_default().push(point),
        _ => {}
    });

    for point in grid.points() {
        for values in antennas.values() {
            for (a1, a2) in values.iter().tuple_combinations() {
                let delta1 = point - a1.as_signed_point();
                let delta2 = point - a2.as_signed_point();

                if delta1 * 2 == delta2 || delta2 * 2 == delta1 {
                    antinodes.insert(point);
                    grid[point.cast()] = Tile::Antinode;
                }
            }
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);
    let mut antinodes = HashSet::new();
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    grid.flat_iter().for_each(|(tile, point)| match tile {
        Tile::Antenna(c) => antennas.entry(*c).or_default().push(point),
        _ => {}
    });

    for point in grid.points() {
        for values in antennas.values() {
            for (a1, a2) in values.iter().tuple_combinations() {
                let delta1 = point - a1.as_signed_point();
                let delta2 = point - a2.as_signed_point();
                for i in 2..100 {
                    for j in 0..100 {
                        if delta1 * i == delta2 * j || delta2 * i == delta1 * j {
                            antinodes.insert(point);
                            grid[point.cast()] = Tile::Antinode;
                            break;
                        }
                    }
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
