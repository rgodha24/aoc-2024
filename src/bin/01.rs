advent_of_code::solution!(1);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i64> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(n1, n2)| (n1.parse::<i64>().unwrap(), n2.parse::<i64>().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right.into_iter())
            .map(|(x, y)| (x - y).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(n1, n2)| (n1.parse::<i64>().unwrap(), n2.parse::<i64>().unwrap()))
        .unzip();

    left.sort();
    let right = right.into_iter().counts();

    Some(
        left.into_iter()
            .map(|n| n * right.get(&n).cloned().unwrap_or(0) as i64)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
