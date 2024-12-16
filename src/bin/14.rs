advent_of_code::solution!(14);
use std::{cmp::Ordering::*, collections::HashSet};

use advent_of_code::helpers::*;
use itertools::Itertools;

fn solve(input: &str, dimensions: SignedPoint) -> usize {
    let mut bots = Vec::new();
    for line in input.lines() {
        let (p, x) = line.split_once(" ").unwrap();
        let p = &p[2..];
        let v = &x[2..];
        let (px, py) = p.split_once(",").unwrap();
        let (px, py) = (px.parse().unwrap(), py.parse().unwrap());
        let (vx, vy) = v.split_once(",").unwrap();
        let (vx, vy) = (vx.parse().unwrap(), vy.parse().unwrap());
        bots.push((SignedPoint::new(px, py), SignedPoint::new(vx, vy)));
    }

    for _ in 0..100 {
        for (botx, botv) in bots.iter_mut() {
            *botx = *botx + *botv;
        }
    }

    let qboundary = dimensions / 2;
    bots.into_iter()
        .map(|(pos, _)| {
            SignedPoint::new(
                pos.x.rem_euclid(dimensions.x),
                pos.y.rem_euclid(dimensions.y),
            )
        })
        .filter_map(|p| match (p.x.cmp(&qboundary.x), p.y.cmp(&qboundary.y)) {
            (Equal, _) | (_, Equal) => None,
            (Less, Less) => Some(1),
            (Less, Greater) => Some(2),
            (Greater, Less) => Some(3),
            (Greater, Greater) => Some(4),
        })
        .counts()
        .values()
        .product()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, SignedPoint::new(101, 103)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let dimensions = SignedPoint::new(101, 103);
    let mut bots = Vec::new();
    for line in input.lines() {
        let (p, x) = line.split_once(" ").unwrap();
        let p = &p[2..];
        let v = &x[2..];
        let (px, py) = p.split_once(",").unwrap();
        let (px, py) = (px.parse().unwrap(), py.parse().unwrap());
        let (vx, vy) = v.split_once(",").unwrap();
        let (vx, vy) = (vx.parse().unwrap(), vy.parse().unwrap());
        bots.push((SignedPoint::new(px, py), SignedPoint::new(vx, vy)));
    }

    if bots.len() != 500 {
        return None;
    }
    for step in 0.. {
        for (botx, botv) in bots.iter_mut() {
            *botx = *botx + *botv;
        }
        let points: HashSet<_> = bots
            .iter()
            .map(|(pos, _)| {
                SignedPoint::new(
                    pos.x.rem_euclid(dimensions.x),
                    pos.y.rem_euclid(dimensions.y),
                )
                .as_point()
                .unwrap()
            })
            .collect();

        // i just kept increasing this number until i eventually got to 500 (every single bot)
        if points.len() >= 500 {
            let grid: Grid<char> = Grid::from(points).map(|b, _| if *b { '#' } else { '.' });
            println!("{step}\n{grid}");
            return Some(step + 1);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(
            &advent_of_code::template::read_file("examples", DAY),
            SignedPoint::new(11, 7),
        );
        assert_eq!(result, (12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
