advent_of_code::solution!(18);
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    steps: usize,
    point: Point,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps).reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

tiles!('.' => Empty, '#' => Corrupted);

fn solve(input: &str, goal: Point, nth: usize) -> Option<usize> {
    let bytes = input
        .lines()
        .map(|line| Point::from_delimited(line, ",").unwrap())
        .collect_vec();
    let grid = std::iter::successors(
        Some((Grid::empty(goal.x + 1, goal.y + 1), 0)),
        |(prev, i)| {
            let b = bytes[i % bytes.len()];
            let mut grid: Grid<Tile> = prev.clone();
            grid[b] = Tile::Corrupted;
            Some((grid, i + 1))
        },
    )
    .map(|(grid, _)| grid)
    .nth(nth)
    .unwrap();

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(State {
        steps: 0,
        point: Point::new(0, 0),
    });

    while let Some(State { steps, point }) = (pq.pop()) {
        if point == goal {
            return Some(steps);
        }
        if !visited.insert(point) {
            continue;
        }
        for point in grid.neighbors_of(point) {
            if matches!(grid[point], Tile::Empty) {
                pq.push(State {
                    steps: steps + 1,
                    point,
                });
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    (solve(input, Point::new(70, 70), 1024))
}

pub fn part_two(input: &str) -> Option<usize> {
    for i in 0.. {
        if solve(input, Point::new(70, 70), i).is_none() {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(
            &advent_of_code::template::read_file("examples", DAY),
            Point::new(6, 6),
            12,
        );
        assert_eq!(result, (22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
