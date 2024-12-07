advent_of_code::solution!(7);
use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut total = 0;
    fn recurse(i: usize, nums: &[usize], total: usize, ans: usize) -> bool {
        if i == nums.len() {
            return ans == total;
        }
        recurse(i + 1, nums, total + nums[i], ans) || recurse(i + 1, nums, total * nums[i], ans)
    }

    for line in input.lines() {
        let (ans, rest) = line.split_once(": ").unwrap();
        let ans = ans.parse().unwrap();
        let nums = line_to_nums(rest).collect_vec();
        if recurse(0, &nums, 0, ans) {
            total += ans;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut total = 0;

    fn recurse(i: usize, nums: &[usize], total: usize, ans: usize) -> bool {
        if i == nums.len() {
            return ans == total;
        }
        recurse(i + 1, nums, total + nums[i], ans)
            || recurse(i + 1, nums, total * nums[i], ans)
            || recurse(
                i + 1,
                nums,
                total * 10usize.pow(nums[i].to_string().len() as u32) + nums[i],
                ans,
            )
    }

    for line in input.lines() {
        let (ans, rest) = line.split_once(": ").unwrap();
        let ans = ans.parse().unwrap();
        let nums = line_to_nums(rest).collect_vec();
        if recurse(0, &nums, 0, ans) {
            total += ans;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
