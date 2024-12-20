advent_of_code::solution!(20);
use std::{collections::BinaryHeap, fmt::Binary};

use advent_of_code::helpers::*;
use itertools::Itertools;

tiles!('.' => Empty, '#' => Wall, 'S' => Start, 'E' => End);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let start = grid.find(Tile::Start).next().unwrap();
    let end = grid.find(Tile::End).next().unwrap();
    let mut fastest: Grid<usize> = grid.empty_sized();
    fastest.fill(usize::MAX);
    let mut q = vec![(0, start)];

    while let Some((cost, point)) = q.pop() {
        if fastest[point] <= cost {
            continue;
        }
        fastest[point] = cost;
        for neighbor in grid.neighbors_of(point) {
            if grid[neighbor] != Tile::Wall {
                q.push((cost + 1, neighbor));
            }
        }
    }

    let start_to_end = fastest[end];

    Some(
        grid.points()
            .flat_map(|point| Direction::all().into_iter().map(move |d| (point, d)))
            .flat_map(|(point, direction)| {
                if grid[point] == Tile::Wall {
                    return None;
                }
                match grid.get(point.as_signed_point() + direction + direction) {
                    None | Some(Tile::Wall) => {
                        // doesn't save us any time
                        None
                    }
                    Some(_) => {
                        let before_cheat_time = fastest[point];
                        let after_cheat_time = fastest[point + direction + direction];

                        if after_cheat_time > start_to_end {
                            Some(
                                (after_cheat_time - start_to_end)
                                    + (before_cheat_time - start_to_end),
                            )
                        } else {
                            (after_cheat_time > before_cheat_time)
                                .then(|| (after_cheat_time - before_cheat_time))
                        }
                    }
                }
            })
            .map(|saved| saved - 2)
            .filter(|saved| *saved >= 100)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let start = grid.find(Tile::Start).next().unwrap();
    let end = grid.find(Tile::End).next().unwrap();
    let mut fastest: Grid<usize> = grid.empty_sized();
    fastest.fill(usize::MAX);
    let mut q = vec![(0, start)];

    while let Some((cost, point)) = q.pop() {
        if fastest[point] <= cost {
            continue;
        }
        fastest[point] = cost;
        for neighbor in grid.neighbors_of(point) {
            if grid[neighbor] != Tile::Wall {
                q.push((cost + 1, neighbor));
            }
        }
    }

    let start_to_end = fastest[end];

    Some(
        dbg!(grid
            .points()
            .flat_map(|point| {
                (-20..=20i64).flat_map(move |x| {
                    (-20..=20i64).filter_map(move |y| {
                        let distance = x.abs() + y.abs();
                        (distance <= 20).then_some((
                            point,
                            point + SignedPoint::new(x, y),
                            distance as usize,
                        ))
                    })
                })
            })
            .flat_map(|(cs, ce, distance)| {
                if grid[cs.cast()] == Tile::Wall {
                    return None;
                }
                match grid.get(ce) {
                    None | Some(Tile::Wall) => {
                        // doesn't save us any time
                        None
                    }
                    Some(_) => {
                        let before_cheat_time = fastest[cs.cast()];
                        let after_cheat_time = fastest[ce.cast()];

                        if after_cheat_time > start_to_end {
                            Some(
                                (after_cheat_time - start_to_end)
                                    + (before_cheat_time - start_to_end)
                                    - distance,
                            )
                        } else {
                            (after_cheat_time > before_cheat_time)
                                .then(|| (after_cheat_time - before_cheat_time - distance))
                        }
                    }
                }
            })
            .filter(|saved| *saved >= 100)
            .counts())
        .values()
        .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
