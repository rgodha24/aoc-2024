advent_of_code::solution!(2);
use std::cmp::Ordering;

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn safe(it: impl Iterator<Item = i32>) -> bool {
    let mut ordering = Ordering::Equal;
    it.tuple_windows().all(|(n1, n2)| {
        (match (n1.cmp(&n2), ordering) {
            // if the first 2 are equal its never safe
            (Ordering::Equal, _) => false,
            // set the ordering to the comparison of the first 2
            (comparison, Ordering::Equal) => {
                ordering = comparison;
                true
            }
            // make sure that 2,3.. have the same ordering as 1,2
            (comparison, ordering) => comparison == ordering,
        }) && (n1.abs_diff(n2) <= 3)
    })
}

pub fn safe_dampened(v: &[i32]) -> bool {
    (0..v.len()).any(|i| {
        safe(
            v.iter()
                // skips the ith object in the iterator
                .enumerate()
                .filter_map(|(j, n)| (j != i).then_some(n))
                // we'd be iterating over &i32 without this
                .copied(),
        )
    })
}

// O(l * n) where l is the number of lines, and n is the numbers per line
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(line_to_nums)
            // .filter() takes &Iterator<Item = i32> as an argument to the function passed into it,
            // so instead we use filter_map and return Some(unit) when the line is safe
            .filter_map(|it| safe(it).then_some(()))
            .count(),
    )
}

// O(l * n * n) where l is the number of lines, and n is the numbers per line
pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            // this allocation is not really avoidable, and the few ways of avoiding it probably
            // would make this run slower than it already does
            .map(|line| line_to_nums(line).collect_vec())
            .filter_map(|v| safe_dampened(&v).then_some(()))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
