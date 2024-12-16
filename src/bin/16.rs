advent_of_code::solution!(16);
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Wall,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            c => panic!("char {c} unknown"),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "{}", 'S'),
            Tile::End => write!(f, "{}", 'E'),
            Tile::Wall => write!(f, "{}", '#'),
            Tile::Empty => write!(f, "{}", '.'),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Grid<Tile> = Grid::from_chars(input);
    let start = grid
        .flat_iter()
        .find_map(|(tile, point)| matches!(tile, Tile::Start).then_some(point))
        .unwrap();
    let end = grid
        .flat_iter()
        .find_map(|(tile, point)| matches!(tile, Tile::End).then_some(point))
        .unwrap();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start, Direction::Right)));
    let mut fastest: HashMap<_, _> = grid
        .points()
        .flat_map(|point| Direction::all().map(|d| (point, d)))
        .map(|k| (k, usize::MAX))
        .collect();

    while let Some(Reverse((cost, point, direction))) = heap.pop() {
        if fastest[&(point, direction)] <= cost {
            continue;
        }
        fastest.insert((point, direction), cost);

        // println!("{}", point);
        if grid
            .get(point + direction)
            .is_some_and(|tile| !matches!(tile, Tile::Wall))
        {
            heap.push(Reverse((cost + 1, point + direction, direction)));
        }

        heap.push(Reverse((cost + 1000, point, direction.right())));
        heap.push(Reverse((cost + 1000, point, direction.left())));
    }

    {
        let mut new_grid = grid.clone();
        for (&(p, dir), &speed) in fastest.iter() {
            if speed != usize::MAX {
                new_grid[p] = Tile::Start;
            }
        }
        // println!("{new_grid}",);
    }

    Direction::all()
        .into_iter()
        .map(|direction| fastest[&(end, direction)])
        .min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let start = grid
        .flat_iter()
        .find_map(|(tile, point)| matches!(tile, Tile::Start).then_some(point))
        .unwrap();
    let end = grid
        .flat_iter()
        .find_map(|(tile, point)| matches!(tile, Tile::End).then_some(point))
        .unwrap();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start, Direction::Right, vec![start])));
    let mut fastest: HashMap<_, _> = grid
        .points()
        .flat_map(|point| Direction::all().map(|d| (point, d)))
        .map(|k| (k, usize::MAX))
        .collect();

    let mut end_paths: HashMap<_, HashSet<_>> = HashMap::new();

    while let Some(Reverse((cost, point, direction, mut path))) = heap.pop() {
        if fastest[&(point, direction)] < cost {
            continue;
        }
        fastest.insert((point, direction), cost);
        path.push(point);

        if point == end {
            let hs = end_paths.entry(cost).or_default();
            hs.extend(path.clone().drain(..));
        }

        // println!("{}", point);
        if grid
            .get(point + direction)
            .is_some_and(|tile| !matches!(tile, Tile::Wall))
        {
            heap.push(Reverse((
                cost + 1,
                point + direction,
                direction,
                path.clone(),
            )));
        }

        heap.push(Reverse((
            cost + 1000,
            point,
            direction.right(),
            path.clone(),
        )));
        heap.push(Reverse((cost + 1000, point, direction.left(), path)));
    }

    {
        let mut new_grid = grid.clone();
        for (&(p, dir), &speed) in fastest.iter() {
            if speed != usize::MAX {
                new_grid[p] = Tile::Start;
            }
        }
        // println!("{new_grid}",);
    }

    let min_cost = Direction::all()
        .into_iter()
        .map(|direction| fastest[&(end, direction)])
        .min()
        .unwrap();

    Some(end_paths[&min_cost].len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
