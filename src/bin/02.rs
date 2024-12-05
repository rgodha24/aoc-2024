advent_of_code::solution!(2);
use std::cmp::Ordering;

use advent_of_code::helpers::*;
use itertools::Itertools;
use smallvec::SmallVec;

fn safe(it: impl Iterator<Item = u8>) -> bool {
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

fn safe_dampened(v: &[u8]) -> bool {
    (0..v.len()).any(|i| {
        safe(
            v.iter()
                // skips the ith object in the iterator
                .enumerate()
                .filter_map(|(j, n)| (j != i).then_some(n))
                // we'd be iterating over &u8 without this
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
            // .filter() takes &Iterator<Item = u8> as an argument to the function passed into it,
            // so instead we use filter_map and return Some(unit) when the line is safe
            .filter_map(|it| safe(it).then_some(()))
            .count(),
    )
}

// O(l * n * n) where l is the number of lines, and n is the numbers per line
pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    // the largest line has 8 numbers on it
    let mut arr: SmallVec<[u8; 8]> = SmallVec::new();

    for line in input.lines() {
        arr.extend(line_to_nums(line));
        if safe_dampened(&arr) {
            sum += 1;
        }
        arr.clear();
    }

    Some(sum)
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
