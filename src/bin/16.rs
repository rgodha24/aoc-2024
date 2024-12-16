advent_of_code::solution!(16);
use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use advent_of_code::helpers::*;

tiles!('S' => Start, 'E' => End, '.' => Empty, '#' => Wall);

fn djikstras(grid: Grid<Tile>) -> Grid<DirectionMap<usize>> {
    let start = grid.find(Tile::Start).unwrap();

    let mut fastest = grid.map(|_, _| DirectionMap::new_cloned(usize::MAX));
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        point: start,
        direction: Direction::Right,
    });

    while let Some(State {
        cost,
        point,
        direction,
    }) = heap.pop()
    {
        let min_cost = &mut fastest[point][direction];
        if *min_cost <= cost {
            continue;
        }
        *min_cost = cost;

        if grid
            .get(point + direction)
            .is_some_and(|tile| *tile != Tile::Wall)
        {
            heap.push(State {
                cost: cost + 1,
                point: point + direction,
                direction,
            });
        }

        heap.push(State {
            cost: cost + 1000,
            point,
            direction: direction.right(),
        });
        heap.push(State {
            cost: cost + 1000,
            point,
            direction: direction.left(),
        });
    }

    fastest
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let end = grid.find(Tile::End).unwrap();
    let fastest = djikstras(grid);

    fastest[end].into_iter().min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let end = grid.find(Tile::End).unwrap();
    let fastest = djikstras(grid);
    let mut visited = HashSet::new();
    fn backtrack(
        fastest: &Grid<DirectionMap<usize>>,
        visited: &mut HashSet<Point>,
        State {
            cost,
            point,
            direction,
        }: State,
    ) {
        if fastest[point][direction] != cost || cost == 0 {
            return;
        }
        visited.insert(point);
        backtrack(
            fastest,
            visited,
            State {
                direction,
                point: point - direction,
                cost: cost - 1,
            },
        );
        backtrack(
            fastest,
            visited,
            State {
                direction: direction.right(),
                point,
                cost: cost - 1000,
            },
        );
        backtrack(
            fastest,
            visited,
            State {
                direction: direction.left(),
                point,
                cost: cost - 1000,
            },
        );
    }

    let lowest_cost = *fastest[end].iter().min().unwrap();
    for direction in Direction::all() {
        backtrack(
            &fastest,
            &mut visited,
            State {
                cost: lowest_cost,
                point: end,
                direction,
            },
        );
    }

    Some(visited.len())
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    point: Point,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
