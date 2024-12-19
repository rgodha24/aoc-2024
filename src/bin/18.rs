advent_of_code::solution!(18);
use std::collections::{BinaryHeap, HashSet};

use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    steps: usize,
    point: Point,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps).reverse().then(
            self.point
                .x
                .cmp(&other.point.x)
                .reverse()
                .then(self.point.y.cmp(&other.point.y).reverse()),
        )
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

tiles!('.' => Empty, '#' => Corrupted);

fn pathfind(grid: &Grid<Tile>) -> Option<usize> {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(State {
        steps: 0,
        point: Point::new(0, 0),
    });
    let goal = grid.dimensions() - Point::new(1, 1);

    while let Some(State { steps, point }) = pq.pop() {
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
    let bytes = input
        .lines()
        .map(|line| Point::from_delimited(line, ",").unwrap())
        .collect_vec();
    let mut grid: Grid<Tile> = Grid::empty(71, 71);
    for i in 0..1024 {
        grid[bytes[i % bytes.len()]] = Tile::Corrupted;
    }
    pathfind(&grid)
}

pub fn part_two(input: &str) -> Option<Point> {
    let bytes = input
        .lines()
        .map(|line| Point::from_delimited(line, ",").unwrap())
        .collect_vec();
    let grid: Grid<Tile> = Grid::empty(71, 71);

    let mut l = 0;
    let mut r = bytes.len() - 1;
    while l < r {
        let m = (r + l) / 2;
        let mut g = grid.clone();
        for b in bytes.iter().take(m).cloned() {
            g[b] = Tile::Corrupted;
        }
        match pathfind(&g) {
            Some(_) => l = m + 1,
            None => r = m - 1,
        }
    }
    Some(bytes[r])
}
