advent_of_code::solution!(15);
use std::{collections::HashSet, fmt::Display};

use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Robot,
    Box,
}
use Tile::*;

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            'O' => Box,
            '@' => Robot,
            '#' => Wall,
            c => panic!("unknown character {c:?}"),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid: Grid<Tile> = Grid::from_chars(grid);
    let moves = moves
        .trim()
        .chars()
        .filter(|&c| (!c.is_whitespace())).map(Direction::from)
        .collect_vec();

    let mut robot = grid
        .flat_iter()
        .find_map(|(tile, point)| (*tile == Robot).then_some(point))
        .unwrap();

    for direction in moves {
        match grid[robot + direction] {
            Empty | Robot => {
                robot += direction;
            }
            Wall => {
                continue;
            }
            Box => {
                let mut first_empty = robot + direction;
                while grid.get(first_empty).is_some_and(|tile| *tile == Box) {
                    first_empty += direction;
                }
                match grid[first_empty] {
                    // e.g. we found an empty tile somewhere ahead of the robot
                    Empty | Robot => {
                        robot += direction;
                        grid.swap(first_empty, robot);
                    }
                    // either the search failed to go forward or we ran into a wall, so we do
                    // nothing
                    Wall | Box => {}
                }
            }
        }
    }

    Some(
        grid.flat_iter()
            .filter_map(|(tile, point)| matches!(tile, Box).then_some(point.x + point.y * 100))
            .sum(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Tile2 {
    BoxL,
    BoxR,
    #[default]
    Empty,
    Wall,
    Robot,
}

impl Display for Tile2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile2::BoxL => write!(f, "["),
            Tile2::BoxR => write!(f, "]"),
            Tile2::Empty => write!(f, "."),
            Tile2::Wall => write!(f, "#"),
            Tile2::Robot => write!(f, "@"),
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid: Grid<Tile> = Grid::from_chars(grid);
    let moves = moves
        .trim()
        .chars()
        .filter(|&c| (!c.is_whitespace())).map(Direction::from)
        .collect_vec();

    let mut robot = grid
        .flat_iter()
        .find_map(|(tile, point)| (*tile == Robot).then_some(point))
        .unwrap();
    robot.x *= 2;

    let mut enlarged_grid: Grid<Tile2> = Grid::empty(grid.width() * 2, grid.height());

    for (tile, point) in grid.flat_iter() {
        let enlarged_point = Point::new(point.x * 2, point.y);
        match tile {
            Box => {
                enlarged_grid[enlarged_point] = Tile2::BoxL;
                enlarged_grid[enlarged_point + Direction::Right] = Tile2::BoxR;
            }
            Wall => {
                enlarged_grid[enlarged_point] = Tile2::Wall;
                enlarged_grid[enlarged_point + Direction::Right] = Tile2::Wall;
            }
            _ => {
                // enlarged_grid is filled with Tile2::Empty by default anways
            }
        }
    }

    for direction in moves {
        let prev = enlarged_grid[robot].clone();
        match enlarged_grid[robot + direction] {
            Tile2::Empty | Tile2::Robot => {
                robot += direction;
            }
            Tile2::Wall => {}
            Tile2::BoxL | Tile2::BoxR => {
                match direction {
                    // copy paste from pt1
                    Direction::Left | Direction::Right => {
                        let mut first_empty = robot + direction;
                        while enlarged_grid
                            .get(first_empty)
                            .is_some_and(|tile| matches!(tile, Tile2::BoxL | Tile2::BoxR))
                        {
                            first_empty += direction;
                        }
                        if enlarged_grid[first_empty] == Tile2::Empty {
                            first_empty -= direction;
                            assert!(robot.x.abs_diff(first_empty.x) % 2 == 0);
                            for i in 0..robot.x.abs_diff(first_empty.x) as i64 {
                                enlarged_grid[(first_empty.cast() + direction
                                    - (direction.as_point() * i))
                                    .cast()] = match (i % 2, direction) {
                                    (1, Direction::Right) | (0, Direction::Left) => Tile2::BoxL,
                                    (0, Direction::Right) | (1, Direction::Left) => Tile2::BoxR,
                                    _ => panic!("how did we get here?"),
                                };
                            }
                            robot += direction;
                            enlarged_grid[robot] = Tile2::Empty;
                        }
                    }
                    Direction::Up | Direction::Down => {
                        let mut boxes = HashSet::new();
                        let mut q = vec![robot + direction];
                        let mut blocked = false;

                        while let Some(point) = q.pop() {
                            assert!(enlarged_grid.contains_point(point));

                            // dbg!(&enlarged_grid[point]);
                            let (p1, p2) = match enlarged_grid[point] {
                                Tile2::BoxL => (point, point + Direction::Right),
                                Tile2::BoxR => (point + Direction::Left, point),
                                Tile2::Wall => {
                                    blocked = true;
                                    break;
                                }
                                Tile2::Empty | Tile2::Robot => {
                                    continue;
                                }
                            };
                            boxes.insert((p1, p2));
                            q.push(p1 + direction);
                            q.push(p2 + direction);
                        }
                        // dbg!(&boxes);

                        if !blocked {
                            let new_boxes: HashSet<_> =
                                boxes.iter().map(|&(bl, _)| bl + direction).collect();

                            for (bl, br) in boxes.into_iter() {
                                enlarged_grid[bl] = Tile2::Empty;
                                enlarged_grid[br] = Tile2::Empty;
                            }

                            for b in new_boxes {
                                enlarged_grid[b] = Tile2::BoxL;
                                enlarged_grid[b + Direction::Right] = Tile2::BoxR;
                            }
                            robot += direction;
                        }
                    }
                }
            }
        }

        enlarged_grid[robot] = Tile2::Robot;
        enlarged_grid[robot] = prev;
    }

    Some(
        enlarged_grid
            .flat_iter()
            .filter_map(|(tile, point)| {
                matches!(tile, Tile2::BoxL).then_some(point.x + point.y * 100)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#,
        Some(618)
    )]
    #[case(
        r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#,
        Some(9021)
    )]
    #[case(
        r#"####
#.O.
#.@.

<<"#,
        Some(104)
    )]
    fn test_part_two(#[case] s: &str, #[case] ans: Option<usize>) {
        assert_eq!(part_two(s), ans);
    }

    #[test]
    fn test_part_one() {
        let s = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let result = part_one(s);
        assert_eq!(result, Some(2028));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }
}
