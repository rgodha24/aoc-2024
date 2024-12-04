advent_of_code::solution!(4);
use advent_of_code::helpers::*;

pub fn part_one(input: &str) -> Option<i64> {
    let grid: Grid<char> = Grid::from_chars(input);

    let dirs = SignedPoint::new(0, 0).neighbors_diag();
    let mut ans = 0;

    for point in grid.points() {
        for dir in dirs {
            for i in 0..4 {
                match (i, grid.get(point + dir * i)) {
                    (0, Some('X')) | (1, Some('M')) | (2, Some('A')) => {}
                    (3, Some('S')) => ans += 1,
                    _ => break,
                };
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid: Grid<char> = Grid::from_chars(input);

    let dirs = SignedPoint::new(0, 0).neighbors_just_diag();
    let mut ans = 0;

    for point in grid.points() {
        if !matches!(grid.get(point), Some('A')) {
            continue;
        }

        let mut m = 0;
        let mut s = 0;
        for c in dirs.iter().filter_map(|&p| grid.get(p + point)) {
            match c {
                'S' => s += 1,
                'M' => m += 1,
                _ => break,
            }
        }

        // extra condition is for these 2 cases:
        // M.S  S.M
        // .A.  .A.
        // S.M  M.S
        // which isn't a X-MAS
        if m == 2 && s == 2 && grid.get(point + dirs[0]) != grid.get(point + dirs[2]) {
            ans += 1;
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
