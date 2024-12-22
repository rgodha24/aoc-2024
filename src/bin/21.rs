advent_of_code::solution!(21);

use std::collections::VecDeque;

use advent_of_code::helpers::*;
use itertools::Itertools;

const NUMPAD: &str = r#"
789
456
123
 0A
"#;

const DIRPAD: &str = r#"
 ^A
<v>
"#;

fn compute_fastest(pad: &str) -> Grid<Grid<Vec<String>>> {
    let pad: Grid<char> = Grid::from_chars(pad.trim());
    let mut output = pad.map(|_, _| pad.empty_sized());
    for (from, to) in pad.points().collect_vec().into_iter().tuple_combinations() {
        if pad[from] == ' ' || pad[to] == ' ' {
            output[from][to] = vec![];
        }
        if from == to {
            output[from][to] = vec!["A".to_string()];
        }
        println!("{from} {to}");

        let mut q = VecDeque::from([(from, String::new())]);
        let mut optimal = usize::MAX;
        let mut possibilities = Vec::new();
        'upper: while let Some((p, mut moves)) = q.pop_front() {
            for direction in Direction::all() {
                let neighbor = p + direction;
                match pad.get(neighbor) {
                    None | Some(' ') => {}
                    Some(_) => {
                        if neighbor == to {
                        } else {
                        }
                        let mut s = s.clone();
                        s.push(direction.into());
                        q.push_back((neighbor.cast(), s));
                    }
                }
            }
        }

        println!("{from} {to} {possibilities:?}");
        output[from][to] = possibilities;
    }

    output
}

pub fn part_one(input: &str) -> Option<usize> {
    let num_fastest = compute_fastest(NUMPAD);
    println!("{:?}", num_fastest);
    let mut sum = 0;
    for l in input.lines() {
        sum += l[..(l.len() - 1)].parse::<usize>().unwrap() * solve(l, 3);
    }
    Some(sum)
}

fn parse(input: &str) -> (Vec<SignedPoint>, SignedPoint) {
    let a_coord = if input.chars().any(|c| c.is_ascii_digit()) {
        SignedPoint::new(2, 3)
    } else {
        SignedPoint::new(2, 0)
    };

    (
        input
            .chars()
            .map(|c| match c {
                '7' => SignedPoint::new(0, 0),
                '8' => SignedPoint::new(1, 0),
                '9' => SignedPoint::new(2, 0),
                '4' => SignedPoint::new(0, 1),
                '5' => SignedPoint::new(1, 1),
                '6' => SignedPoint::new(2, 1),
                '1' => SignedPoint::new(0, 2),
                '2' => SignedPoint::new(1, 2),
                '3' => SignedPoint::new(2, 2),
                '0' => SignedPoint::new(1, 3),
                '^' => SignedPoint::new(1, 0),
                '<' => SignedPoint::new(0, 1),
                'v' => SignedPoint::new(1, 1),
                '>' => SignedPoint::new(2, 1),
                'A' => a_coord,
                c => panic!("unknown char {c}"),
            })
            .collect_vec(),
        a_coord,
    )
}

fn direction_as_coord(direction: Direction) -> SignedPoint {
    match direction {
        Up => SignedPoint::new(1, 0),
        Left => SignedPoint::new(0, 1),
        Down => SignedPoint::new(1, 1),
        Right => SignedPoint::new(2, 1),
    }
}

fn distance(from: SignedPoint, to: SignedPoint, depth: usize, activate: SignedPoint) -> usize {
    let gap = activate - SignedPoint::new(2, 0);
    if from == gap || to == gap {
        return usize::MAX;
    }
    if depth == 0 {
        return (from.manhattan_distance(&to).unsigned_abs()) as usize;
    }

    let delta = to - from;
    let (direction, amt) = match (delta.x == 0, delta.y == 0) {
        (true, true) => {
            // e.g. from == to
            return 0;
        }
        (true, false) => {
            let direction = if delta.y > 0 { Up } else { Down };
            (direction, delta.y.abs() as usize)
        }
        (false, true) => {
            let direction = if delta.x > 0 { Left } else { Right };
            (direction, delta.x.abs() as usize)
        }
        (false, false) => {
            let a = from + SignedPoint::new(delta.x, 0);
            let b = from + SignedPoint::new(0, delta.y);
            let a_cost = (distance(from, a, depth, activate))
                .checked_add(distance(a, to, depth, activate))
                .unwrap_or(usize::MAX);
            let b_cost = (distance(from, b, depth, activate))
                .checked_add(distance(b, to, depth, activate))
                .unwrap_or(usize::MAX);

            return a_cost.min(b_cost);
        }
    };
    // println!("direction {direction} amt {amt}");
    let coord = direction_as_coord(direction);

    // if we're going downwards in depth, activate is always in the numpad position
    distance(activate, coord, depth - 1, SignedPoint::new(0, 2))
        + amt
        + distance(coord, activate, depth - 1, SignedPoint::new(0, 2))
}

fn solve(s: &str, depth: usize) -> usize {
    let (coords, a) = parse(s);

    println!("activate = {a}");

    coords
        .into_iter()
        .circular_tuple_windows()
        .inspect(|(from, to)| println!("inputting from {from} to {to}!"))
        .map(|(from, to)| dbg!(distance(from, to, depth - 1, a) + 1))
        .sum()
}

use Direction::*;

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("<A", "v<<A>>^A".len(), 1)]
    #[case("<^A", "v<<A>^A>A".len(), 1)]
    #[case("<A^A>^^AvvvA", "v<<A>>^A<A>AvA<^AA>Av<AAA>^A".len(), 1)]
    #[case("v<<A>>^A<A>AvA<^AA>A<vAAA>^A", "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(), 1)]
    #[case("<A^A>^^AvvvA", "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(), 2)]
    #[case("0A", "<A>A".len(), 1)]
    #[case("0A", "v<<A>^^AvA^A".len(), 2)]
    #[case("029A", 28, 2)]
    #[case("029A", 68, 3)]
    #[case("980A", 60, 3)]
    #[case("179A", 68, 3)]
    #[case("456A", 64, 3)]
    #[case("379A", 64, 3)]
    fn test_solve(#[case] input: &str, #[case] output_len: usize, #[case] depth: usize) {
        assert_eq!(solve(input, depth), output_len);
    }
}
