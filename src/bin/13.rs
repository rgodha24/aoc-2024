advent_of_code::solution!(13);
use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Game {
    a: SignedPoint,
    b: SignedPoint,
    goal: SignedPoint,
}

impl Game {
    fn tokens_to_solve(self) -> Option<i64> {
        // shoutout wolfram alpha
        let a = (self.b.y * self.goal.x - self.b.x * self.goal.y)
            / (self.a.x * self.b.y - self.b.x * self.a.y);
        let b = (self.goal.x * self.a.y - self.a.x * self.goal.y)
            / (self.b.x * self.a.y - self.a.x * self.b.y);
        (self.a * a + self.b * b == self.goal).then_some(a * 3 + b)
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
            a: SignedPoint::new(ax, ay),
            b: SignedPoint::new(bx, by),
            goal: SignedPoint::new(prizex, prizey),
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
    let extra = SignedPoint::new(10000000000000, 10000000000000);
    for game in input.split("\n\n") {
        let (a, b, goal) = game.lines().collect_tuple().unwrap();
        let (ax, ay) = a[9..].split_once(", ").unwrap();
        let (ax, ay) = (ax[2..].parse().unwrap(), ay[2..].parse().unwrap());
        let (bx, by) = b[9..].split_once(", ").unwrap();
        let (bx, by) = (bx[2..].parse().unwrap(), by[2..].parse().unwrap());
        let (prizex, prizey) = goal[7..].split_once(", ").unwrap();
        let (prizex, prizey) = (prizex[2..].parse().unwrap(), prizey[2..].parse().unwrap());

        games.push(Game {
            a: SignedPoint::new(ax, ay),
            b: SignedPoint::new(bx, by),
            goal: SignedPoint::new(prizex, prizey) + extra,
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
