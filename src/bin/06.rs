advent_of_code::solution!(6);

use std::{collections::HashSet, fmt::Display, num::NonZeroUsize};

use advent_of_code::helpers::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Obstacle,
    Start,
    None,
    Visited(HashSet<Direction>),
}

impl Tile {
    /// returns true if the tile has been visited in this direction already
    fn visit(&mut self, direction: Direction) -> bool {
        match self {
            Tile::Visited(ref mut hs) => !hs.insert(direction),
            Tile::Obstacle => panic!("trying to visit an obstacle"),
            _ => {
                *self = Tile::Visited(HashSet::from([direction]));
                false
            }
        }
    }

    fn is_visited(&self) -> bool {
        matches!(self, Tile::Visited(_))
    }

    fn is_obstacle(&self) -> bool {
        matches!(self, Tile::Obstacle)
    }

    fn is_start(&self) -> bool {
        matches!(self, Tile::Start)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Tile::None)
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
            Some(t) if t.is_obstacle() => {
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
    let (mut grid, start) = parse(input);

    follow_path(&mut grid, start, Direction::Up, |_, _, _| {})
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut grid, start) = parse(input);
    let mut cycled = 0;

    follow_path(
        &mut grid,
        start,
        Direction::Up,
        |grid, point, &direction| {
            if grid.get(point + direction).is_some_and(|t| t.is_empty()) {
                let mut grid = grid.clone();
                grid[point.cast() + direction] = Tile::Obstacle;

                let path = follow_path(&mut grid, point.cast(), direction.right(), |_, _, _| {});
                // if a cycle is detected:
                if path.is_none() {
                    cycled += 1;
                }
            }
        },
    );

    Some(cycled)
}

fn parse(input: &str) -> (Grid<Tile>, Point) {
    let grid: Grid<Tile> = Grid::from_chars(input);

    let start = grid
        .flat_iter()
        .find_map(|(item, point)| item.is_start().then_some(point))
        .unwrap();

    (grid, start)
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
