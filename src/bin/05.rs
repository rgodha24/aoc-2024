advent_of_code::solution!(5);
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<usize, HashSet<usize>> = Default::default();
    for o in ordering.lines() {
        // y depends on x
        let (x, y) = o
            .split_once("|")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();

        map.entry(y).or_default().insert(x);
    }

    let mut sum = 0;
    'upper: for u in updates.lines() {
        let mut visited = HashSet::new();
        let nums = u.split(",").map(|n| n.parse().unwrap()).collect_vec();
        visited.insert(nums[0]);
        for n in nums.iter().skip(1) {
            dbg!(n);
            let Some(prereqs) = map.get(n) else {
                continue 'upper;
            };
            if !visited.iter().all(|v| prereqs.contains(v)) {
                continue 'upper;
            }
            visited.insert(*n);
        }
        sum += nums[nums.len() / 2];
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<usize, HashSet<usize>> = Default::default();
    for o in ordering.lines() {
        // y depends on x
        let (x, y) = o
            .split_once("|")
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();

        map.entry(y).or_default().insert(x);
    }

    let mut incorrect = Vec::new();
    'upper: for u in updates.lines() {
        let mut visited = HashSet::new();
        let nums = u.split(",").map(|n| n.parse().unwrap()).collect_vec();
        visited.insert(nums[0]);
        for n in nums.iter().skip(1) {
            let Some(prereqs) = map.get(n) else {
                incorrect.push(nums);
                continue 'upper;
            };
            if !visited.iter().all(|v| prereqs.contains(v)) {
                incorrect.push(nums);
                continue 'upper;
            }
            visited.insert(*n);
        }
    }

    let mut sum = 0;
    for nums in incorrect {
        let nums_set: HashSet<_> = nums.iter().cloned().collect();
        dbg!(&nums, &nums_set);

        let mut prereqs = nums
            .iter()
            .map(|n| {
                let Some(prereqs) = map.get(n) else {
                    return (n, HashSet::new());
                };
                (n, prereqs.iter().filter(|n| nums_set.contains(n)).collect())
            })
            .collect_vec();

        let mut new_nums = HashSet::new();
        for i in 0..nums.len() {
            let (p, (n, _)) = prereqs
                .iter()
                .enumerate()
                .find(|(_, (_, hs))| hs.is_subset(&new_nums))
                .unwrap();

            new_nums.insert(n);
            if i == nums.len() / 2 {
                sum += *n;
                break;
            }
            prereqs.remove(p);
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
