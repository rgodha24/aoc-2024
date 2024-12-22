advent_of_code::solution!(22);
use std::collections::HashMap;

use itertools::Itertools;

fn iter_secret(secret: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(secret), |prev| {
        let mut secret = *prev;
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;

        Some(secret)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| iter_secret(l.parse().unwrap()).nth(2000).unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let buyers: Vec<HashMap<_, _>> = input
        .lines()
        .map(|l| {
            let mut hm = HashMap::new();
            for (k, v) in iter_secret(l.parse().unwrap())
                .map(|n| n as i64 % 10)
                .tuple_windows()
                .map(|(a, b, c, d, e)| ((b - a, c - b, d - c, e - d), e))
                .take(2000)
            {
                if hm.contains_key(&k) {
                    continue;
                }
                hm.insert(k, v);
            }
            hm
        })
        .collect_vec();

    itertools::iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)
        .map(|diffs| {
            buyers
                .iter()
                .map(|hm| (hm.get(&diffs).cloned().unwrap_or(0)))
                .sum()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let expected = r#"15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254"#;
        let expected = expected.lines().map(|l| l.parse().unwrap()).collect_vec();
        let iter = iter_secret(123);
        assert_eq!(iter.skip(1).take(expected.len()).collect_vec(), expected);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let s = r#"1
2
3
2024"#;
        let result = part_two(s);
        assert_eq!(result, Some(23));
    }
}
