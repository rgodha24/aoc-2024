advent_of_code::solution!(13);
use advent_of_code::helpers::*;
use itertools::Itertools;
use mini_matrix::Matrix;
use num::Rational64;

type P = GenericPoint<Rational64>;
#[derive(Debug, Clone)]
struct Game {
    a: P,
    b: P,
    goal: P,
}

impl Game {
    fn tokens_to_solve(self) -> Option<i64> {
        let mat = Matrix::from([
            [self.a.x, self.b.x, self.goal.x],
            [self.a.y, self.b.y, self.goal.y],
        ]);

        let rref = mat.row_echelon();
        if rref[(0, 0)] == 1.into() && rref[(1, 1)] == 1.into() {
            let apress = rref[(0, 2)];
            let bpress = rref[(1, 2)];
            (apress.is_integer() && bpress.is_integer())
                .then_some(apress.numer() * 3 + bpress.numer())
        } else {
            None
        }
    }
}
pub fn part_one(input: &str) -> Option<i64> {
    let mut games = Vec::new();
    for game in input.split("\n\n") {
        let (a, b, goal) = game.lines().collect_tuple().unwrap();
        let (ax, ay) = a[9..].split_once(", ").unwrap();
        let (ax, ay) = (ax[2..].parse().unwrap(), ay[2..].parse().unwrap());
        let (bx, by) = b[9..].split_once(", ").unwrap();
        let (bx, by) = (bx[2..].parse().unwrap(), by[2..].parse().unwrap());
        let (prizex, prizey) = goal[7..].split_once(", ").unwrap();
        let (prizex, prizey) = (prizex[2..].parse().unwrap(), prizey[2..].parse().unwrap());

        games.push(Game {
            a: P::new(ax, ay),
            b: P::new(bx, by),
            goal: P::new(prizex, prizey),
        });
    }

    Some(
        games
            .into_iter()
            .filter_map(|game| game.tokens_to_solve())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut games = Vec::new();
    let extra = P::new(10000000000000.into(), 10000000000000.into());
    for game in input.split("\n\n") {
        let (a, b, goal) = game.lines().collect_tuple().unwrap();
        let (ax, ay) = a[9..].split_once(", ").unwrap();
        let (ax, ay) = (ax[2..].parse().unwrap(), ay[2..].parse().unwrap());
        let (bx, by) = b[9..].split_once(", ").unwrap();
        let (bx, by) = (bx[2..].parse().unwrap(), by[2..].parse().unwrap());
        let (prizex, prizey) = goal[7..].split_once(", ").unwrap();
        let (prizex, prizey) = (prizex[2..].parse().unwrap(), prizey[2..].parse().unwrap());

        games.push(Game {
            a: P::new(ax, ay),
            b: P::new(bx, by),
            goal: P::new(prizex, prizey) + extra,
        });
    }

    Some(
        games
            .into_iter()
            .filter_map(|game| game.tokens_to_solve())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
