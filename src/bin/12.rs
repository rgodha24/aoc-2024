advent_of_code::solution!(12);
use std::collections::HashSet;

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::from_chars(input);
    let mut ans = 0;
    let mut visited = HashSet::new();
    for p in grid.points() {
        let p: Point = p;
        let p = p.as_signed_point();
        if !visited.insert(p) {
            continue;
        }
        let mut area = HashSet::new();
        let mut q = vec![p];
        let c = grid[p.cast()];
        while let Some(point) = q.pop() {
            if grid[point.cast()] != c {
                continue;
            }
            if !area.insert(point) {
                continue;
            }
            visited.insert(point);
            for neighbor in grid.neighbors_of(point.cast()) {
                q.push(neighbor.cast());
            }
        }

        let perimeter = area
            .iter()
            .flat_map(|point| point.neighbors())
            .filter(|p| !area.contains(p))
            .count();
        ans += area.len() * perimeter;
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::from_chars(input);
    let mut ans = 0;
    let mut visited = HashSet::new();
    for p in grid.points() {
        let p: Point = p;
        let p = p.as_signed_point();
        if !visited.insert(p) {
            continue;
        }
        let mut area = HashSet::new();
        let mut q = vec![p];
        let c = grid[p.cast()];
        while let Some(point) = q.pop() {
            if grid[point.cast()] != c {
                continue;
            }
            if !area.insert(point) {
                continue;
            }
            visited.insert(point);
            for neighbor in grid.neighbors_of(point.cast()) {
                q.push(neighbor.cast());
            }
        }

        let corner_directions = Direction::all().map(|d| (d, d.right()));

        let corners: usize = area
            .iter()
            .flat_map(|p| p.neighbors_diag().into_iter())
            .filter(|p| !area.contains(p))
            .unique()
            .map(|p| {
                let mut corners = 0;
                for &(d1, d2) in corner_directions.iter() {
                    // this point is a corner in this direction if area contains p+(d1, d2) OR p+(!d1, !d2, d1+d2)
                    let c1 = area.contains(&(p + d1));
                    let c2 = area.contains(&(p + d2));
                    let c3 = area.contains(&(p + d1 + d2));
                    match (c1, c2, c3) {
                        (true, true, _) | (false, false, true) => corners += 1,
                        _ => {}
                    }
                }
                corners
            })
            .sum();

        // the number of corners == the number of sides. I might be dumb
        ans += area.len() * corners;
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));

        let s = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
        let result = part_two(s);
        assert_eq!(result, Some(236));

        let s = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        let result = part_two(s);
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_s3() {
        let s = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
        let result = part_two(s);
        assert_eq!(result, Some(368));
    }
}
