advent_of_code::solution!(3);
use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<i64> {
    let re = Regex::new(r#"mul\(\d+,\d+\)"#).expect("regex compiles");
    let mut sum = 0;
    for s in re.find_iter(input) {
        let s = s.as_str().replace("mul(", "").replace(")", "");
        let (n1, n2) = s.split_once(",").unwrap();
        let n1: i64 = n1.parse().unwrap();
        let n2: i64 = n2.parse().unwrap();

        sum += n1 * n2;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let re = Regex::new(r#"mul\(\d+,\d+\)"#).expect("regex compiles");
    let mut sum = 0;

    let dos = Regex::new(r#"do\(\)"#)
        .expect("regex")
        .find_iter(input)
        .map(|m| m.start())
        .collect_vec();
    let donts = Regex::new(r#"don\'t\(\)"#)
        .expect("regex")
        .find_iter(input)
        .map(|m| m.start())
        .collect_vec();

    for m in re.find_iter(input) {
        let s = m.as_str().replace("mul(", "").replace(")", "");
        let (n1, n2) = s.split_once(",").unwrap();
        let n1: i64 = n1.parse().unwrap();
        let n2: i64 = n2.parse().unwrap();

        let start = m.start();
        let max_do = dos.iter().filter(|&&d| d < start).max().cloned();
        let max_dont = donts.iter().filter(|&&d| d < start).max().cloned();
        match (max_do, max_dont) {
            (Some(a), Some(b)) if a > b => {
                sum += n1 * n2;
            }
            (_, None) => {
                sum += n1 * n2;
            }
            _ => {}
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
