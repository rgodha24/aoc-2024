advent_of_code::solution!(6);

use std::{collections::HashSet, fmt::Display};

use advent_of_code::helpers::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Obstacle,
    Start,
    None,
    // only really for debugging purposes
    Visited,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::None,
            '^' => Self::Start,
            '#' => Self::Obstacle,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Obstacle => write!(f, "#"),
            Tile::Start | Tile::None => write!(f, "."),
            Tile::Visited => write!(f, "X"),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);

    let (_, point) = grid
        .flat_iter()
        .find(|(item, _)| **item == Tile::Start)
        .unwrap();
    let mut point = point.as_signed_point();
    let mut visited = HashSet::new();
    let mut direction = Direction::Up;
    visited.insert(point);

    while grid.contains_point(point) {
        visited.insert(point);
        grid[point.cast()] = Tile::Visited;

        match grid.get(point + direction) {
            Some(Tile::Obstacle) => {
                direction.turn_right();
            }
            Some(_) => {
                point += &direction;
            }
            None => return Some(visited.len()),
        }
    }

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);

    let (_, start) = grid
        .flat_iter()
        .find(|(item, _)| **item == Tile::Start)
        .unwrap();
    let mut point = start.as_signed_point();
    let mut direction = Direction::Up;

    while grid.contains_point(point) {
        grid[point.cast()] = Tile::Visited;

        match grid.get(point + direction) {
            Some(Tile::Obstacle) => {
                direction.turn_right();
            }
            Some(_) => {
                point += &direction;
            }
            None => break,
        }
    }

    let old_grid = grid.clone();
    let mut total = 0;
    'upper: for change_point in old_grid
        .flat_iter()
        .filter_map(|(&item, point)| (item == Tile::Visited).then_some(point))
    {
        grid[change_point] = Tile::Obstacle;

        let mut point = start.as_signed_point();
        let mut direction = Direction::Up;

        let mut visited = HashSet::new();
        let mut iterations = 0;

        while grid.contains_point(point) && iterations <= visited.len() * 2 + 1 {
            grid[point.cast()] = Tile::Visited;
            visited.insert(point);
            iterations += 1;

            match grid.get(point + direction) {
                Some(Tile::Obstacle) => {
                    direction.turn_right();
                }
                Some(_) => {
                    point += &direction;
                }
                None => {
                    grid[change_point] = Tile::Visited;
                    continue 'upper;
                }
            }
        }

        grid[change_point] = Tile::Visited;
        total += 1;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
