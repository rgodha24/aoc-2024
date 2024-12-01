advent_of_code::solution!(1);
use std::collections::{HashMap, HashSet};

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i64> {
    let l: Vec<(i64, i64)> = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(n1, n2)| (n1.parse().unwrap(), n2.parse().unwrap()))
        .collect();

    let mut v1 = l.iter().map(|(x, y)| x).collect_vec();
    let mut v2 = l.iter().map(|(x, y)| y).collect_vec();
    v1.sort();
    v2.sort();
    Some(
        v1.into_iter()
            .zip(v2.iter())
            .map(|(x, y)| (*x - *y).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let l: Vec<(i64, i64)> = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(n1, n2)| (n1.parse().unwrap(), n2.parse().unwrap()))
        .collect();

    let mut v1 = l.iter().map(|(x, y)| x).collect_vec();
    let mut v2: HashMap<i64, i64> = Default::default();

    for (_, n) in l.clone() {
        *v2.entry(n).or_default() += 1;
    }

    println!("{:?}", v2);

    Some(
        v1.into_iter()
            .map(|n| n * v2.get(n).cloned().unwrap_or(0))
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
