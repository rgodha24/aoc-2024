advent_of_code::solution!(7);
use advent_of_code::helpers::*;
use smallvec::SmallVec;

fn recurse<const APPEND_OP: bool>(i: usize, nums: &[isize], total: isize) -> bool {
    if i == 0 {
        return nums[0] == total;
    }
    if nums[i] >= total {
        return false;
    }

    if recurse::<APPEND_OP>(i - 1, nums, total - nums[i]) {
        return true;
    }

    if total % nums[i] == 0 && recurse::<APPEND_OP>(i - 1, nums, total / nums[i]) {
        return true;
    }

    // bc this is a const generic, this gets optimized away in p1
    if APPEND_OP {
        let digits = nums[i].checked_ilog10().unwrap() + 1;
        if (total % 10isize.pow(digits)) == nums[i] {
            return recurse::<APPEND_OP>(i - 1, nums, total / 10isize.pow(digits));
        }
    }

    false
}

fn solve<const APPEND_OP: bool>(input: &str) -> isize {
    input
        .lines()
        .filter_map(|line| {
            let (ans, rest) = line.split_once(": ").unwrap();
            let ans = ans.parse().unwrap();
            let nums: SmallVec<[_; 16]> = line_to_nums(rest).collect();

            recurse::<APPEND_OP>(nums.len() - 1, &nums, ans).then_some(ans)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(solve::<false>(input))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(solve::<true>(input))
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
