advent_of_code::solution!(11);
use std::collections::{HashMap, HashSet};

use advent_of_code::helpers::*;
use itertools::Itertools;

fn digits(n: usize) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut nums: HashMap<usize, usize> = HashMap::new();
    for n in input.split_whitespace() {
        let n: usize = n.parse().unwrap();
        *nums.entry(n).or_default() += 1;
    }
    for _ in 0..25 {
        println!("{:?}", nums);
        let mut new_nums: HashMap<usize, usize> = HashMap::new();
        for (k, v) in nums.iter() {
            if *k == 0 {
                *new_nums.entry(1).or_default() += v;
            } else if digits(*k) % 2 == 0 {
                let as_str = k.to_string();
                println!("{as_str}");
                let h1 = as_str[..as_str.len() / 2].parse().unwrap();
                let h2 = as_str[as_str.len() / 2..].parse().unwrap();
                *new_nums.entry(h1).or_default() += v;
                *new_nums.entry(h2).or_default() += v;
            } else {
                *new_nums.entry(k * 2024).or_default() += v;
            }
        }
        nums = new_nums;
    }

    Some(nums.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut nums: HashMap<usize, usize> = HashMap::new();
    for n in input.split_whitespace() {
        let n: usize = n.parse().unwrap();
        *nums.entry(n).or_default() += 1;
    }
    for _ in 0..75 {
        println!("{:?}", nums);
        let mut new_nums: HashMap<usize, usize> = HashMap::new();
        for (k, v) in nums.iter() {
            if *k == 0 {
                *new_nums.entry(1).or_default() += v;
            } else if digits(*k) % 2 == 0 {
                let as_str = k.to_string();
                println!("{as_str}");
                let h1 = as_str[..as_str.len() / 2].parse().unwrap();
                let h2 = as_str[as_str.len() / 2..].parse().unwrap();
                *new_nums.entry(h1).or_default() += v;
                *new_nums.entry(h2).or_default() += v;
            } else {
                *new_nums.entry(k * 2024).or_default() += v;
            }
        }
        nums = new_nums;
    }

    Some(nums.values().sum())
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
