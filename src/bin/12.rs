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
        println!(
            "point {p} with char {c} area {:?} and perimeter {}",
            area, perimeter
        );
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

        // let top_left = area
        //     .iter()
        //     .flat_map(|point| point.neighbors_diag())
        //     .filter(|p| !area.contains(p))
        //     .min_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
        //     .unwrap();
        //
        // dbg!(top_left);
        let mut master = HashSet::new();
        let mut sides = 0;
        for x in -1..=grid.width() as i64 {
            for y in -1..=grid.height() as i64 {
                let point = SignedPoint::new(x, y);
                if area.contains(&point) {
                    continue;
                }
                if !master.insert(point) {
                    continue;
                }
                let mut floodfill = HashSet::new();
                floodfill.insert(point);
                let mut q = vec![point];
                while let Some(p) = q.pop() {
                    if floodfill.insert(p) {
                        continue;
                    }
                    for neighbor in p.neighbors() {
                        if !area.contains(&neighbor) {
                            q.push(neighbor);
                        }
                    }
                }
                dbg!(&floodfill);
                let Some(mut p) = area
                    .iter()
                    .flat_map(|p| p.neighbors().into_iter())
                    .find(|p| floodfill.contains(p))
                else {
                    continue;
                };
                let mut directions = Vec::new();
                let mut direction = Direction::all()
                    .into_iter()
                    .find(|&d| area.contains(&(p + d.right())))
                    .unwrap();
                assert!(area.contains(&(p + direction.right())));
                let start = (p, direction);
                loop {
                    dbg!((p, direction, c));
                    directions.push(direction);
                    if area.contains(&(p + direction)) {
                        direction = direction.left();
                        p += direction;
                        sides += 1;
                    } else {
                        if area.contains(&(p + direction.right())) {
                            p += direction;
                        } else {
                            direction = direction.right();
                            p += direction;
                            sides += 1;
                        }
                    }
                    if p == start.0 {
                        break;
                    }
                }
                if directions.last() == directions.first() {
                    sides -= 1;
                }
            }
        }
        // while sides_visited.insert(currpoint) {
        // dbg!(&currpoint, c);
        // if perimeter.contains(&(currpoint + direction)) {
        //     currpoint += direction;
        // } else {
        //     for d in direction.except_self() {
        //         if perimeter.contains(&(currpoint + d))
        //             && !sides_visited.contains(&(currpoint + d))
        //         {
        //             direction = d;
        //             sides += 1;
        //             currpoint += direction;
        //             break;
        //         }
        //     }
        // }
        // }

        // println!("point {p} with char {c} area {:?} and perimeter {}", area, perimeter );
        ans += area.len() * sides;
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
    }
}
