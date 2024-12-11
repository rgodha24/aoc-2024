advent_of_code::solution!(10);
use std::collections::{BinaryHeap, HashSet, VecDeque};

use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Tile(u32);
impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile(value.to_digit(10).unwrap())
    }
}
pub fn part_one(input: &str) -> Option<i64> {
    let grid: Grid<Tile> = Grid::from_chars(input);

    let mut score = 0;
    for (t, zero_point) in grid.flat_iter().filter(|(tile, point)| tile.0 == 0) {
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        let mut nines = 0;
        q.push_back(zero_point);

        while let Some(p) = q.pop_front() {
            if !visited.insert(p) {
                continue;
            }
            if grid[p] == Tile(9) {
                nines += 1;
            }
            for neighbor in grid.neighbors_of(p) {
                if grid[neighbor].0 != grid[p].0 + 1 {
                    continue;
                }
                q.push_back(neighbor);
            }
        }

        score += nines;
    }

    Some(score)
}

fn recurse(grid: &Grid<Tile>, point: Point) -> usize {
    if grid[point] == Tile(9) {
        return 1;
    }
    let mut ans = 0;
    for neighbor in grid.neighbors_of(point) {
        if grid[neighbor].0 == grid[point].0 + 1 {
            ans += recurse(grid, neighbor);
        }
    }

    ans
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);

    let mut score = 0;
    for (t, zero_point) in grid.flat_iter().filter(|(tile, point)| tile.0 == 0) {
        score += recurse(&grid, zero_point);
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
