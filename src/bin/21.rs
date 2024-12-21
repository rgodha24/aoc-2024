advent_of_code::solution!(21);
use std::fmt::Display;

use advent_of_code::helpers::*;
use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy)]
enum Buttons {
    Direction(Direction),
    Activate,
}
use Direction::*;

impl Display for Buttons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Buttons::Direction(d) => d.fmt(f),
            Buttons::Activate => write!(f, "A"),
        }
    }
}

fn as_coords(buttons: Vec<Buttons>) -> String {
    buttons.into_iter().map(|b| b.to_string()).collect()
}

fn iterate(buttons: &str, first: bool) -> String {
    let a_coord = if first {
        SignedPoint::new(2, 3)
    } else {
        SignedPoint::new(2, 0)
    };
    let gap = a_coord - SignedPoint::new(2, 0);
    let mut output = Vec::new();
    let mut curr = a_coord;
    for (i, c) in buttons.chars().enumerate() {
        if (i != 0 && curr == a_coord - SignedPoint::new(2, 0)) || curr.x < 0 || curr.y < 0 {
            panic!("how did we get here??");
        }

        let c = match c {
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
        };
        let has_gap_problem = !first && curr.manhattan_distance(&c) == 4;
        if has_gap_problem {
            println!("{curr} to {c} has a gap problem");
        }
        let delta = curr - c;

        if has_gap_problem {
            if first {
                if delta.y > 0 {
                    output.extend(repeat_n(Buttons::Direction(Up), delta.y as usize));
                }
                if delta.x > 0 {
                    output.extend(repeat_n(Buttons::Direction(Left), delta.x as usize));
                }
                if delta.x < 0 {
                    output.extend(repeat_n(Buttons::Direction(Right), (-delta.x) as usize));
                }
                if delta.y < 0 {
                    output.extend(repeat_n(Buttons::Direction(Down), (-delta.y) as usize));
                }
            } else {
                if delta.y < 0 {
                    output.extend(repeat_n(Buttons::Direction(Down), (-delta.y) as usize));
                }
                if delta.x > 0 {
                    output.extend(repeat_n(Buttons::Direction(Left), delta.x as usize));
                }
                if delta.x < 0 {
                    output.extend(repeat_n(Buttons::Direction(Right), (-delta.x) as usize));
                }
                if delta.y > 0 {
                    output.extend(repeat_n(Buttons::Direction(Up), delta.y as usize));
                }
            }
        } else {
            if delta.x < 0 {
                output.extend(repeat_n(Buttons::Direction(Right), (-delta.x) as usize));
            }
            if delta.y > 0 {
                output.extend(repeat_n(Buttons::Direction(Up), delta.y as usize));
            }
            if delta.y < 0 {
                output.extend(repeat_n(Buttons::Direction(Down), (-delta.y) as usize));
            }
            if delta.x > 0 {
                output.extend(repeat_n(Buttons::Direction(Left), delta.x as usize));
            }
        }
        output.push(Buttons::Activate);
        curr = c;
    }

    as_coords(output)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for l in input.lines() {
        // println!("{l}");
        // let coords = l.chars().map(|c| match c {}).collect_vec();
        //
        // let buttons = iterate(coords, SignedPoint::new(2, 3), true);
        // for b in &buttons {
        //     print!("{b}");
        // }
        // println!("   len({})", buttons.len());
        // // println!("");
        // // let c = as_coords(buttons.clone());
        // // for c in c {
        // //     print!("{c} ");
        // // }
        // let buttons = iterate(as_coords(buttons), SignedPoint::new(2, 0), false);
        // for b in &buttons {
        //     print!("{b}");
        // }
        // println!("   len({})", buttons.len());
        // let buttons = iterate(as_coords(buttons), SignedPoint::new(2, 0), false);
        // for b in &buttons {
        //     print!("{b}");
        // }
        // println!("   len({})", buttons.len());
        //
        // sum += l[..(l.len() - 1)].parse::<usize>().unwrap() * buttons.len()
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
    #[case("029A", "<A^A^^>AvvvA")]
    fn test_first(#[case] input: &str, #[case] output: &str) {
        assert_eq!(&iterate(input, true), output);
    }

    #[rstest]
    #[case("<A^A>^^AvvvA", "v<<A>>^A<A>AvA<^AA>Av<AAA>^A")]
    fn test_second(#[case] input: &str, #[case] output: &str) {
        println!("input: {input} && expected: {output}");
        assert_eq!(&iterate(input, false), output);
    }

    #[rstest]
    #[case("029A", 68)]
    #[case("980A", 60)]
    #[case("179A", 68)]
    #[case("456A", 64)]
    #[case("379A", 64)]
    fn test_full(#[case] input: &str, #[case] output_len: usize) {
        println!("{input}. expected output length: {output_len}");
        let iter_1 = iterate(input, true);
        println!("{iter_1}");
        let iter_2 = iterate(&iter_1, false);
        println!("{iter_2}");
        let iter_3 = iterate(&iter_2, false);
        println!("{iter_3}");
        assert_eq!(iter_3.len(), output_len);
    }
}
