advent_of_code::solution!(21);

use std::collections::HashMap;

use advent_of_code::helpers::*;
use cached::proc_macro::cached;
use itertools::Itertools;

const NUMPAD: &str = r#"789
456
123
 0A
"#;

const DIRPAD: &str = r#" ^A
<v>
"#;
type Fastest = HashMap<(GenericPoint<usize>, GenericPoint<usize>), Vec<String>>;

fn compute_fastest(pad: &str) -> Fastest {
    use Direction::*;
    let pad: Grid<char> = Grid::from_chars(pad);
    let mut output = HashMap::new();
    for from in pad.points() {
        for to in pad.points() {
            if pad[from.cast()] == ' ' || pad[to.cast()] == ' ' {
                continue;
            }
            if from == to {
                output.insert((from.cast(), to.cast()), vec!["A".to_string()]);
                continue;
            }

            let delta: SignedPoint = from - to;
            let x = (if delta.x < 0 { Right } else { Left }, delta.x.abs());
            let y = (if delta.y < 0 { Down } else { Up }, delta.y.abs());

            let x = x.0.to_string().repeat(x.1 as usize);
            let y = y.0.to_string().repeat(y.1 as usize);
            let choices = [format!("{x}{y}A"), format!("{y}{x}A")];
            if choices[0] == choices[1] {
                output.insert((from.cast(), to.cast()), vec![choices[0].clone()]);
                continue;
            }

            let v = choices
                .into_iter()
                .filter(|path| !intersects_gap(path, from.cast(), &pad))
                .collect_vec();

            output.insert((from.cast(), to.cast()), v);
        }
    }

    output
}

fn intersects_gap(path: &str, mut point: Point, grid: &Grid<char>) -> bool {
    // remove the A
    let path = &path[..path.len() - 1];
    for d in path.chars().map(Direction::from) {
        point += d;
        if grid[point] == ' ' {
            return true;
        }
    }
    false
}

#[cached]
fn num_moves(s: String, depth: usize) -> usize {
    let (parsed, fastest) = parse(&s);
    if depth == 0 {
        return s.len();
    }

    parsed
        .into_iter()
        .circular_tuple_windows()
        .map(|(from, to)| {
            fastest[&(from.cast(), to.cast())]
                .iter()
                .map(|path| num_moves(path.to_string(), depth - 1))
                .min()
                .expect("fastest.len() != 0")
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| (num_moves(l.to_string(), 3)) * (l[..l.len() - 1].parse::<usize>().unwrap()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| (num_moves(l.to_string(), 26)) * (l[..l.len() - 1].parse::<usize>().unwrap()))
            .sum(),
    )
}

fn parse(input: &str) -> (Vec<SignedPoint>, Fastest) {
    let (a_coord, fastest) = if input.chars().any(|c| c.is_ascii_digit()) {
        (SignedPoint::new(2, 3), compute_fastest(NUMPAD))
    } else {
        (SignedPoint::new(2, 0), compute_fastest(DIRPAD))
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
        fastest,
    )
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
        assert_eq!(num_moves(input, depth), output_len);
    }

    #[rstest]
    #[case("<<v", Point::new(2, 0), DIRPAD, true)]
    #[case("v<<", Point::new(2, 0), DIRPAD, false)]
    fn test_intersects_gap(
        #[case] path: &str,
        #[case] start: Point,
        #[case] pad: &str,
        #[case] intersects: bool,
    ) {
        assert_eq!(
            intersects_gap(path, start, &Grid::from_chars(pad)),
            intersects
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
