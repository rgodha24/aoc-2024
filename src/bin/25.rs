advent_of_code::solution!(25);
use advent_of_code::helpers::*;
use either::Either::{Left, Right};
use itertools::Itertools;

tiles!('#' => Filled, '.' => Empty);

pub fn part_one(input: &str) -> Option<usize> {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(|grid| Grid::<Tile>::from_chars(grid))
        .partition_map(|grid| {
            let heights = grid
                .cols()
                .map(|v| v.into_iter().filter(|t| matches!(t, Tile::Filled)).count())
                .collect_vec();

            if matches!(grid[Point::new(0, 0)], Tile::Empty) {
                Right(heights)
            } else {
                Left(heights)
            }
        });

    Some(
        itertools::iproduct!(locks, keys)
            .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(l, k)| l + k < 8))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
