advent_of_code::solution!(22);

use bitvec_simd::BitVec;
use itertools::Itertools;

fn secrets(secret: u64) -> impl Iterator<Item = u64> {
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
            .map(|l| secrets(l.parse().unwrap()).nth(2000).unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u16> {
    const fn as_index((a, b, c, d): (i64, i64, i64, i64)) -> usize {
        (a + 9) as usize * 19usize.pow(4)
            + (b + 9) as usize * 19usize.pow(3)
            + (c + 9) as usize * 19usize.pow(2)
            + (d + 9) as usize * 19usize.pow(1)
    }
    const SIZE: usize = as_index((9, 9, 9, 9));
    let mut prices = Box::new([0; SIZE]);
    let mut visited = BitVec::zeros(SIZE);

    for l in input.lines() {
        visited.set_all_false();
        for (k, v) in secrets(l.parse().unwrap())
            .map(|n| n as i64 % 10)
            .tuple_windows()
            .map(|(a, b, c, d, e)| ((b - a, c - b, d - c, e - d), e as u16))
            .take(2000)
        {
            let i = as_index(k);
            if !visited.get(i).unwrap() {
                prices[i] += v;
                visited.set(i, true);
            }
        }
    }

    prices.into_iter().max()
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
        let iter = secrets(123);
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
