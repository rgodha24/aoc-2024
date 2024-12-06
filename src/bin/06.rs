advent_of_code::solution!(6);

use advent_of_code::helpers::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tile(u8);

impl Tile {
    pub const OBSTACLE: u8 = 0b10000;
    pub const DIRECTIONS: u8 = 0b1111;

    // `Start` is also considered empty
    pub const START: u8 = 0b1100000;
    pub const EMPTY: u8 = 0b1000000;

    /// returns true if the tile has been visited in this direction already
    fn visit(&mut self, direction: Direction) -> bool {
        let visited = self.0 & (direction as u8) > 0;
        self.0 = (self.0 & Self::DIRECTIONS) | (direction as u8);
        return visited;
    }

    fn is_visited(&self) -> bool {
        self.0 & Self::DIRECTIONS > 0
    }

    fn is_obstacle(&self) -> bool {
        self.0 & Self::OBSTACLE > 0
    }

    fn is_start(&self) -> bool {
        self.0 == Self::START
    }

    fn is_empty(&self) -> bool {
        self.0 & Self::EMPTY > 0
    }
}

type TileGrid = Grid<Tile, 130>;

#[derive(Debug, PartialEq, Eq)]
enum PathResult {
    Cycle,
    OffTheGrid, // by Kanye West???
}
use itertools::Itertools;
use PathResult::*;

/// returns None if the grid cycles, and returns Some(visited) if the path goes off the grid
fn follow_path(
    grid: &mut TileGrid,
    start: Point,
    mut direction: Direction,
    mut for_each: impl FnMut((&TileGrid, &SignedPoint, &Direction)),
) -> PathResult {
    let mut point = start.as_signed_point();
    loop {
        if grid[point.cast()].visit(direction) {
            return Cycle;
        }

        match grid.get(point + direction) {
            Some(t) if t.is_obstacle() => {
                direction = direction.right();
            }
            Some(_) => {
                for_each((&grid, &point, &direction));
                point += direction;
            }
            None => {
                return OffTheGrid;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut grid, start) = parse(input);

    let res = follow_path(&mut grid, start, Direction::Up, |_| {});
    assert_eq!(res, OffTheGrid);

    Some(
        grid.into_inner()
            .into_iter()
            .flat_map(|v| v.into_iter())
            .filter(|t| t.is_visited())
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut grid, start) = parse(input);
    let mut cycled = 0;

    let mut empty = grid.clone();
    let obstacles = grid
        .flat_iter()
        .filter_map(|(tile, point)| tile.is_obstacle().then_some(point))
        .collect_vec();

    follow_path(
        &mut grid,
        start,
        Direction::Up,
        |(grid, &point, &direction)| {
            if grid.get(point + direction).is_some_and(|t| t.is_empty()) {
                empty.fill(Tile(Tile::EMPTY));
                for &o in &obstacles {
                    empty[o] = Tile(Tile::OBSTACLE);
                }
                empty[point.cast() + direction] = Tile(Tile::OBSTACLE);

                if follow_path(&mut empty, point.cast(), direction.right(), |_| {}) == Cycle {
                    cycled += 1;
                }
            }
        },
    );

    Some(cycled)
}

fn parse(input: &str) -> (TileGrid, Point) {
    let grid: TileGrid = Grid::from_chars(input);

    let start = grid
        .flat_iter()
        .find_map(|(item, point)| item.is_start().then_some(point))
        .unwrap();

    (grid, start)
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile(Tile::EMPTY),
            '^' => Tile(Tile::START),
            '#' => Tile(Tile::OBSTACLE),
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_tile_logic() {
        let mut tile = Tile(Tile::START);
        assert_eq!(tile.is_start(), true);
        assert_eq!(tile.is_empty(), true);
        assert_eq!(tile.is_visited(), false);
        assert_eq!(tile.is_obstacle(), false);

        let res = tile.visit(Direction::Right);
        assert_eq!(res, false);
        assert_eq!(tile.is_start(), false);
        assert_eq!(tile.is_empty(), false);
        assert_eq!(tile.is_visited(), true);
        assert_eq!(tile.is_obstacle(), false);
        assert_eq!(tile.0, Direction::Right as u8);

        let res = tile.visit(Direction::Right);
        assert_eq!(res, true);
        assert_eq!(tile.is_start(), false);
        assert_eq!(tile.is_empty(), false);
        assert_eq!(tile.is_visited(), true);
        assert_eq!(tile.is_obstacle(), false);
        assert_eq!(tile.0, Direction::Right as u8);

        let res = tile.visit(Direction::Left);
        assert_eq!(res, false);
        assert_eq!(tile.is_start(), false);
        assert_eq!(tile.is_empty(), false);
        assert_eq!(tile.is_visited(), true);
        assert_eq!(tile.is_obstacle(), false);
        assert_eq!(tile.0, (Direction::Right as u8) + (Direction::Left as u8));
    }
}
