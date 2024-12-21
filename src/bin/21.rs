advent_of_code::solution!(21);

use advent_of_code::helpers::*;
use itertools::Itertools;

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

fn distance(from: SignedPoint, to: SignedPoint, depth: usize) -> usize {
    const GAP: SignedPoint = SignedPoint::new(0, 0);
    const A: SignedPoint = SignedPoint::new(2, 0);
    if from == GAP || to == GAP {
        return usize::MAX;
    }
    if depth == 0 {
        return from.manhattan_distance(&to).unsigned_abs() as usize;
    }

    let delta = from - to;
    let (direction, amt) = match (delta.x == 0, delta.y == 0) {
        (true, true) => {
            // e.g. from == to
            return 0;
        }
        (true, false) => {
            let direction = if delta.y > 0 { Down } else { Up };
            (direction, delta.y.abs() as usize)
        }
        (false, true) => {
            let direction = if delta.x > 0 { Right } else { Left };
            (direction, delta.x.abs() as usize)
        }
        (false, false) => {
            let (a, b) = (
                { direction_as_coord(if delta.x > 0 { Right } else { Left }) },
                { direction_as_coord(if delta.y > 0 { Down } else { Up }) },
            );

            let a_cost = distance(from, a, depth) + distance(a, b, depth);
            let b_cost = distance(from, b, depth) + distance(b, a, depth);

            return a_cost.min(b_cost);
        }
    };
    let coord = direction_as_coord(direction);

    distance(A, coord, depth - 1) + amt + distance(coord, A, depth - 1)
}

fn solve(s: &str, depth: usize) -> usize {
    let (coords, a) = parse(s);

    if a == SignedPoint::new(2, 3) {
        // e.g. 379A
        todo!()
    } else {
        // e.g <<^A
        coords
            .into_iter()
            .tuple_windows()
            .map(|(from, to)| distance(from, to, depth))
            .sum()
    }
}

use Direction::*;

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for l in input.lines() {
        sum += l[..(l.len() - 1)].parse::<usize>().unwrap() * solve(l, 3);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("029A", "<A^A^^>AvvvA".len(), 1)]
    #[case("<A^A>^^AvvvA", "v<<A>>^A<A>AvA<^AA>Av<AAA>^A".len(), 1)]
    #[case("029A", 68, 3)]
    #[case("980A", 60, 3)]
    #[case("179A", 68, 3)]
    #[case("456A", 64, 3)]
    #[case("379A", 64, 3)]
    fn test_solve(#[case] input: &str, #[case] output_len: usize, #[case] depth: usize) {
        assert_eq!(solve(input, depth), output_len);
    }
    //
    // #[rstest]
    // #[case("029A", "<A^A^^>AvvvA")]
    // fn test_first(#[case] input: &str, #[case] expected: &str) {
    //     let output = iterate(input, true);
    //     println!("input: {input} && expected: {expected} && got: {output}");
    //     assert_eq!(output.len(), expected.len());
    // }
    //
    // #[rstest]
    // #[case("<A^A>^^AvvvA", "v<<A>>^A<A>AvA<^AA>Av<AAA>^A")]
    // fn test_second(#[case] input: &str, #[case] expected: &str) {
    //     let output = iterate(input, false);
    //     println!("input: {input} && expected: {expected} && got: {output}");
    //     assert_eq!(output.len(), expected.len());
    // }
    //
    // #[rstest]
    // #[case("029A", 68)]
    // #[case("980A", 60)]
    // #[case("179A", 68)]
    // #[case("456A", 64)]
    // #[case("379A", 64)]
    // fn test_full(#[case] input: &str, #[case] output_len: usize) {
    //     println!("{input}. expected output length: {output_len}");
    //     let iter_1 = iterate(input, true);
    //     println!("{iter_1}");
    //     let iter_2 = iterate(&iter_1, false);
    //     println!("{iter_2}");
    //     let iter_3 = iterate(&iter_2, false);
    //     println!("{iter_3}");
    //     assert_eq!(iter_3.len(), output_len);
    // }
}
