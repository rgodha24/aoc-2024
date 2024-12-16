advent_of_code::solution!(11);
use advent_of_code::helpers::*;
use std::collections::HashMap;

fn solve(input: &str, steps: usize) -> u64 {
    let mut nums: HashMap<u64, u64> = HashMap::new();
    for n in input.split_whitespace() {
        let n: u64 = n.parse().unwrap();
        *nums.entry(n).or_default() += 1;
    }
    for _ in 0..steps {
        let mut new_nums: HashMap<u64, u64> = HashMap::new();
        for (k, v) in nums.iter() {
            if *k == 0 {
                *new_nums.entry(1).or_default() += v;
            } else if digits(*k) % 2 == 0 {
                let d = 10u64.pow(digits(*k) / 2);
                let h1 = k / d;
                let h2 = k % d;
                *new_nums.entry(h1).or_default() += v;
                *new_nums.entry(h2).or_default() += v;
            } else {
                *new_nums.entry(k * 2024).or_default() += v;
            }
        }
        nums = new_nums;
    }

    nums.values().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
