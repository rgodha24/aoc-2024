advent_of_code::solution!(10);
use std::collections::HashSet;

use advent_of_code::helpers::*;

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<u8> = Grid::from_chars(input);

    let mut score = 0;
    for zero_point in grid.find(b'0') {
        let mut visited = HashSet::new();
        score += recurse(&grid, zero_point, &mut Some(&mut visited));
    }

    Some(score)
}

/// solves day10 using a dfs. uses the visited set if its passed in (only useful for pt1)
///
/// &mut Option<&mut T> is horrifically ugly but thats the only way the borrow checker is happy
fn recurse(grid: &Grid<u8>, point: Point, visited: &mut Option<&mut HashSet<Point>>) -> usize {
    if let Some(visited) = visited {
        if !visited.insert(point) {
            return 0;
        }
    }
    if grid[point] == b'9' {
        return 1;
    }
    let mut ans = 0;
    for neighbor in grid.neighbors_of(point) {
        if grid[neighbor] == grid[point] + 1 {
            ans += recurse(grid, neighbor, visited);
        }
    }

    ans
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<u8> = Grid::from_chars(input);

    let mut score = 0;
    for zero_point in grid.find(b'0') {
        score += recurse(&grid, zero_point, &mut None);
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
