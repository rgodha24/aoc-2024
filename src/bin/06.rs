advent_of_code::solution!(6);

use std::{collections::HashSet, fmt::Display, num::NonZeroUsize};

use advent_of_code::helpers::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Obstacle,
    Start,
    None,
    Visited,
}

/// returns None if the grid cycles, and returns Some(visited) if the path goes off the grid
fn follow_path(grid: &mut Grid<Tile>, start: Point, direction: Direction) -> Option<NonZeroUsize> {
    todo!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);

    let (_, start) = grid
        .flat_iter()
        .find(|(item, _)| **item == Tile::Start)
        .unwrap();

    let mut point = start.as_signed_point();
    let mut direction = Direction::Up;

    loop {
        grid[point.cast()] = Tile::Visited;

        match grid.get(point + direction) {
            Some(Tile::Obstacle) => {
                direction = direction + Direction::Right;
            }
            Some(_) => {
                point += direction;
            }
            None => break,
        }
    }

    Some(
        grid.into_inner()
            .into_iter()
            .flat_map(|v| v.into_iter())
            .filter_map(|t| matches!(t, Tile::Visited).then_some(()))
            .count(),
    )
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
                direction = direction + Direction::Right;
            }
            Some(_) => {
                point += direction;
            }
            None => break,
        }
    }

    let old_grid = grid.clone();
    let mut total = 0;
    'upper: for change_point in old_grid
        .flat_iter()
        .filter_map(|(item, point)| matches!(item, Tile::Visited).then_some(point))
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
                    direction = direction + Direction::Right;
                }
                Some(_) => {
                    point += direction;
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
