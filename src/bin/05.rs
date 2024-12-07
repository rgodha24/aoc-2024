advent_of_code::solution!(5);
use std::cmp::Ordering;

use smallvec::SmallVec;

fn parse(
    input: &str,
) -> (
    [u128; 90],
    impl Iterator<Item = SmallVec<[usize; 24]>> + use<'_>,
) {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let mut arr = [0; 90];
    let ordering = ordering.as_bytes();
    // each step will have `dd|ddn` where d is a digit, and n is a \n, which is 6 bytes long
    for i in (0..ordering.len()).step_by(6) {
        let x = (ordering[i] - b'0') * 10 + ordering[i + 1] - b'0';
        // the extra -1 is to change `arr[y-10]` to `arr[y]`
        let y = (ordering[i + 3] - b'0' - 1) * 10 + ordering[i + 4] - b'0';
        arr[y as usize] |= 1 << x;
    }

    (
        arr,
        updates
            .lines()
            .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect()),
    )
}

fn is_safe(v: &[usize], prereqs: &[u128]) -> bool {
    for i in 1..v.len() {
        for j in 0..i {
            // check if j has a prereq on i
            if (prereqs[v[j] - 10] & (0b1 << v[i])) != 0 {
                // we only want unsorted ones
                return false;
            }
        }
    }

    return true;
}

pub fn part_one(input: &str) -> Option<usize> {
    let (prereqs, updates) = parse(input);

    Some(
        updates
            .filter_map(|v| is_safe(&v, &prereqs).then_some(v[v.len() / 2]))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (prereqs, updates) = parse(input);

    Some(
        updates
            .filter(|v| !is_safe(v, &prereqs))
            .map(|mut v| {
                v.sort_unstable_by(|a, b| {
                    if (prereqs[a - 10] & (0b1 << b)) != 0 {
                        // a has prereq on b, so a > b
                        Ordering::Greater
                    } else if (prereqs[b - 10] & (0b1 << a)) != 0 {
                        // b has prereq on a, so a < b
                        Ordering::Less
                    } else {
                        // no prereqs so theyre equal
                        Ordering::Equal
                    }
                });

                v[v.len() / 2]
            })
            .sum(),
    )
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
