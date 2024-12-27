advent_of_code::solution!(25);
use advent_of_code::helpers::*;
use either::Either::{Left, Right};
use itertools::Itertools;

tiles!('#' => Filled, '.' => Empty);

pub fn part_one(input: &str) -> Option<i64> {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(|grid| Grid::<Tile>::from_chars(grid))
        .partition_map(|grid| {
            let heights = grid
                .cols()
                .map(|v| v.into_iter().filter(|t| matches!(t, Tile::Filled)).count() - 1)
                .collect_vec();
            dbg!(&heights);
            if matches!(grid[Point::new(0, 0)], Tile::Empty) {
                Right(heights.into_iter().map(|h| grid.height() - h).collect_vec())
            } else {
                Left(heights)
            }
        });

    let mut ans = 0;
    for l in locks {
        for k in &keys {
            if l.iter().zip(k.into_iter()).all(|(l, k)| l != k) {
                println!(
                    "lock {} key {}",
                    l.iter().map(|n| n.to_string()).join(", "),
                    k.iter().map(|n| n.to_string()).join(", ")
                );
                ans += 1;
            }
        }
    }

    Some(ans)
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
