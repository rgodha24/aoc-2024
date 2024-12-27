advent_of_code::solution!(21);

use std::collections::HashMap;

use advent_of_code::helpers::*;
use itertools::{Either, Itertools};

const NUMPAD: &str = r#"789
456
123
 0A
"#;

const DIRPAD: &str = r#" ^A
<v>
"#;
type Fastest =
    HashMap<(GenericPoint<usize>, GenericPoint<usize>), Either<[String; 2], [String; 1]>>;

fn compute_fastest(pad: &str) -> Fastest {
    use Direction::*;
    let pad: Grid<char> = Grid::from_chars(pad);
    let mut output = HashMap::new();
    for from in pad.points() {
        for to in pad.points() {
            if pad[from] == ' ' || pad[to] == ' ' {
                continue;
            }
            if from == to {
                output.insert((from, to), Either::Right(["A".to_string()]));
                continue;
            }

            let delta: SignedPoint = from.cast() - to.cast();
            let x = (if delta.x < 0 { Right } else { Left })
                .to_string()
                .repeat(delta.x.abs() as usize);
            let y = (if delta.y < 0 { Down } else { Up })
                .to_string()
                .repeat(delta.y.abs() as usize);

            let a = format!("{x}{y}A");
            let b = format!("{y}{x}A");

            let path = match (
                intersects_gap(&a, from, &pad),
                intersects_gap(&b, from, &pad),
            ) {
                (true, true) => panic!("both paths intersect gap, which should be impossible"),
                (false, true) => Either::Right([a]),
                (true, false) => Either::Right([b]),
                (false, false) => Either::Left([a, b]),
            };

            output.insert((from, to), path);
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

fn num_moves(s: String, depth: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    if let Some(moves) = cache.get(&(s.clone(), depth)) {
        return *moves;
    }
    let (parsed, fastest) = parse(&s);
    if depth == 0 {
        return s.len();
    }

    let moves = parsed
        .into_iter()
        .circular_tuple_windows()
        .map(|(from, to)| {
            fastest[&(from, to)]
                .as_ref()
                .into_iter()
                .into_inner()
                .map(|path| num_moves(path.to_string(), depth - 1, cache))
                .min()
                .expect("fastest.len() != 0")
        })
        .sum();

    cache.insert((s, depth), moves);
    moves
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    Some(
        input
            .trim()
            .lines()
            .map(|l| {
                (num_moves(l.to_string(), 3, &mut cache))
                    * (l[..l.len() - 1].parse::<usize>().unwrap())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    Some(
        input
            .trim()
            .lines()
            .map(|l| {
                (num_moves(l.to_string(), 26, &mut cache))
                    * (l[..l.len() - 1].parse::<usize>().unwrap())
            })
            .sum(),
    )
}

fn parse(input: &str) -> (Vec<Point>, Fastest) {
    let (a_coord, fastest) = if input.chars().any(|c| c.is_ascii_digit()) {
        (Point::new(2, 3), compute_fastest(NUMPAD))
    } else {
        (Point::new(2, 0), compute_fastest(DIRPAD))
    };

    (
        input
            .chars()
            .map(|c| match c {
                '7' => Point::new(0, 0),
                '8' => Point::new(1, 0),
                '9' => Point::new(2, 0),
                '4' => Point::new(0, 1),
                '5' => Point::new(1, 1),
                '6' => Point::new(2, 1),
                '1' => Point::new(0, 2),
                '2' => Point::new(1, 2),
                '3' => Point::new(2, 2),
                '0' => Point::new(1, 3),
                '^' => Point::new(1, 0),
                '<' => Point::new(0, 1),
                'v' => Point::new(1, 1),
                '>' => Point::new(2, 1),
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
        let mut cache = HashMap::new();
        assert_eq!(num_moves(input.to_string(), depth, &mut cache), output_len);
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
