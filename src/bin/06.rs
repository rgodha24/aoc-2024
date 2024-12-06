advent_of_code::solution!(6);

use std::{collections::HashSet, fmt::Display, num::NonZeroUsize};

use advent_of_code::helpers::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Obstacle,
    NewObstacle,
    Start,
    None,
    Visited(HashSet<Direction>),
}

impl Tile {
    /// returns true if the tile has been visited in this direction already
    fn visit(&mut self, direction: Direction) -> bool {
        match self {
            Tile::Visited(ref mut hs) => !hs.insert(direction),
            Tile::Obstacle | Tile::NewObstacle => panic!("trying to visit an obstacle"),
            _ => {
                *self = Tile::Visited(HashSet::from([direction]));
                false
            }
        }
    }

    fn is_visited(&self) -> bool {
        matches!(self, Tile::Visited(_))
    }
}

/// returns None if the grid cycles, and returns Some(visited) if the path goes off the grid
fn follow_path(
    grid: &mut Grid<Tile>,
    start: Point,
    mut direction: Direction,
    mut for_each: impl FnMut(&Grid<Tile>, SignedPoint, &Direction),
) -> Option<NonZeroUsize> {
    let mut point = start.as_signed_point();
    loop {
        if grid[point.cast()].visit(direction) {
            // cycle
            return None;
        }

        match grid.get(point + direction) {
            Some(Tile::Obstacle | Tile::NewObstacle) => {
                direction = direction.right();
            }
            Some(_) => {
                for_each(&grid, point, &direction);
                point += direction;
            }
            None => {
                // off the grid
                return NonZeroUsize::new(
                    grid.iter()
                        .flat_map(|v| v.iter())
                        .filter_map(|t| t.is_visited().then_some(()))
                        .count(),
                );
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<NonZeroUsize> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);

    let (_, start) = grid
        .flat_iter()
        .find(|(item, _)| **item == Tile::Start)
        .unwrap();

    follow_path(&mut grid, start, Direction::Up, |_, _, _| {})
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);

    let (_, start) = grid
        .flat_iter()
        .find(|(item, _)| **item == Tile::Start)
        .unwrap();

    let mut cycled = HashSet::new();
    follow_path(&mut grid, start, Direction::Up, |grid, point, direction| {
        let mut grid = grid.clone();
        if !grid.contains_point(point + *direction) || grid[point.cast() + *direction].is_visited()
        {
            return;
        }
        grid[point.cast() + *direction] = Tile::NewObstacle;
        let path = follow_path(&mut grid, point.cast(), direction.right(), |_, _, _| {});
        if path.is_none() {
            cycled.insert(point + *direction);
        }
    });
    Some(cycled.len())
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
            Tile::NewObstacle => write!(f, "O"),
            Tile::Start | Tile::None => write!(f, "."),
            Tile::Visited(_) => write!(f, "X"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, NonZeroUsize::new(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1284791287492));
    }
}
