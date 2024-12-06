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
}

/// returns None if the grid cycles, and returns Some(visited) if the path goes off the grid
fn follow_path(
    grid: &mut Grid<Tile>,
    start: Point,
    mut direction: Direction,
    mut for_each: impl FnMut(&Grid<Tile>, Point, &Direction),
) -> Option<NonZeroUsize> {
    let mut point = start;
    loop {
        if grid[point.cast()].visit(direction) {
            // cycle
            return None;
        }

        match grid.get(point + direction) {
            Some(Tile::Obstacle) => {
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

pub fn part_two(input: &str) -> Option<i64> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);

    let (_, start) = grid
        .flat_iter()
        .find(|(item, _)| **item == Tile::Start)
        .unwrap();

    let mut cycles = 0;
    follow_path(&mut grid, start, Direction::Up, |grid, point, direction| {
        let mut grid = grid.clone();
        println!("{grid}");
        if !grid.contains_point(point + *direction) {
            return;
        }
        grid[point + *direction] = Tile::Obstacle;
        dbg!(point, direction);
        println!("{} {}", grid[point], grid[point + *direction]);
        println!("{grid}");
        let path = follow_path(&mut grid, point, direction.right(), |_, _, _| {});
        println!("{grid}{:?}", path);
        if path.is_none() {
            cycles += 1;
        }
    });
    Some(cycles)
    // let mut grid: Grid<Tile> = Grid::from_chars(input);
    //
    //
    // let (_, start) = grid
    //     .flat_iter()
    //     .find(|(item, _)| **item == Tile::Start)
    //     .unwrap();
    // let mut point = start.as_signed_point();
    // let mut direction = Direction::Up;
    //
    // while grid.contains_point(point) {
    //     grid[point.cast()] = Tile::Visited;
    //
    //     match grid.get(point + direction) {
    //         Some(Tile::Obstacle) => {
    //             direction = direction + Direction::Right;
    //         }
    //         Some(_) => {
    //             point += direction;
    //         }
    //         None => break,
    //     }
    // }
    //
    // let old_grid = grid.clone();
    // let mut total = 0;
    // 'upper: for change_point in old_grid
    //     .flat_iter()
    //     .filter_map(|(item, point)| matches!(item, Tile::Visited).then_some(point))
    // {
    //     grid[change_point] = Tile::Obstacle;
    //
    //     let mut point = start.as_signed_point();
    //     let mut direction = Direction::Up;
    //
    //     let mut visited = HashSet::new();
    //     let mut iterations = 0;
    //
    //     while grid.contains_point(point) && iterations <= visited.len() * 2 + 1 {
    //         grid[point.cast()] = Tile::Visited;
    //         visited.insert(point);
    //         iterations += 1;
    //
    //         match grid.get(point + direction) {
    //             Some(Tile::Obstacle) => {
    //                 direction = direction + Direction::Right;
    //             }
    //             Some(_) => {
    //                 point += direction;
    //             }
    //             None => {
    //                 grid[change_point] = Tile::Visited;
    //                 continue 'upper;
    //             }
    //         }
    //     }
    //
    //     grid[change_point] = Tile::Visited;
    //     total += 1;
    // }
    //
    // Some(total)
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
        assert_eq!(result, Some(6));
    }
}
