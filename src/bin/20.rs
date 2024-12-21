advent_of_code::solution!(20);

use advent_of_code::helpers::*;

tiles!('.' => Empty, '#' => Wall, 'S' => Start, 'E' => End);

fn solve(input: &str, cheat_distance: i64, threshold: usize) -> usize {
    let grid: Grid<Tile, 142> = Grid::from_chars(input);
    let start = grid.find(Tile::Start).next().unwrap();
    let end = grid.find(Tile::End).next().unwrap();
    let mut fastest = grid.empty_sized();
    fastest.fill(usize::MAX);
    let mut q = vec![(0, start)];

    while let Some((cost, point)) = q.pop() {
        if fastest[point] <= cost {
            continue;
        }
        fastest[point] = cost;
        for neighbor in grid.neighbors_of(point) {
            if grid[neighbor] != Tile::Wall && fastest[neighbor] > cost + 1 {
                q.push((cost + 1, neighbor));
            }
        }
    }

    let ste = fastest[end];

    grid.points()
        .filter(|p| matches!(&grid[p.cast()], Tile::Empty | Tile::Start))
        .flat_map(move |point| {
            (-cheat_distance..=cheat_distance).flat_map(move |x| {
                let max_y = cheat_distance - x.abs();
                (-max_y..=max_y).map(move |y| {
                    (
                        point,
                        point + SignedPoint::new(x, y),
                        (y.abs() + x.abs()) as usize,
                    )
                })
            })
        })
        .filter_map(|(cheat_start, cheat_end, distance)| {
            if grid[cheat_start.cast()] == Tile::Wall {
                return None;
            }
            match grid.get(cheat_end) {
                None | Some(Tile::Wall) => None,
                Some(_) => {
                    let bct = fastest[cheat_start.cast()];
                    let act = fastest[cheat_end.cast()];

                    if act > ste {
                        // if we cheated to a spot further away from start than the distance, the
                        // distance from us to the end is (act - ste), and the distance from the cheat
                        // start point is (bct - ste). the sum of those 2 numbers, minus the
                        // distance, is the amount of time we've saved with this cheat
                        Some((act - ste) + (bct - ste) - distance)
                    } else {
                        // return how much time we saved as long as it's positive
                        act.checked_sub(bct + distance)
                    }
                }
            }
        })
        .filter(|saved| *saved >= threshold)
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 2, 100))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 20, 100))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(2, 1, 44)] // given in problem statement
    #[case(2, 20, 5)]
    #[case(2, 25, 4)]
    #[case(20, 25, 1027)]
    #[case(20, 40, 593)]
    #[case(20, 50, 285)] // given in problem statement
    #[case(20, 60, 129)]
    #[case(20, 70, 41)]
    #[case(20, 80, 0)]
    fn test_day_20(#[case] distance: i64, #[case] threshold: usize, #[case] amount: usize) {
        let s = &advent_of_code::template::read_file("examples", DAY);
        let count = solve(s, distance, threshold);
        assert_eq!(count, amount);
    }
}
