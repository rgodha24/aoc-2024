advent_of_code::solution!(19);
use std::collections::HashMap;

use advent_of_code::helpers::*;
use itertools::Itertools;

fn patterns<'a>(
    towels: &'a [&'a str],
    need: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if need.len() == 0 {
        return 1;
    }
    if let Some(n) = cache.get(need) {
        return *n;
    }
    let mut total = 0;
    for t in towels {
        if need.starts_with(t) {
            total += patterns(towels, &need[t.len()..], cache);
        }
    }

    cache.insert(need, total);
    total
}

pub fn part_one(input: &str) -> Option<usize> {
    let (have, want) = input.split_once("\n\n").unwrap();
    let towels = have.split(", ").collect_vec();
    let mut cache = HashMap::new();
    Some(
        want.lines()
            .filter(|line| patterns(&towels, line, &mut cache) != 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (have, want) = input.split_once("\n\n").unwrap();
    let towels = have.split(", ").collect_vec();
    let mut cache = HashMap::new();
    Some(
        want.lines()
            .map(|line| patterns(&towels, line, &mut cache))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
