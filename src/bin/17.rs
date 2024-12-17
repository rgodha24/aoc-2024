advent_of_code::solution!(17);
use std::str::FromStr;

use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    instructions: (u64, usize),
}
impl Computer {
    fn solve(&self) -> (u64, usize) {
        let Computer {
            mut a,
            mut b,
            mut c,
            instructions,
        } = self;
        let (instructions, count) = *instructions;

        let mut i = 0;
        let mut output = 0u64;
        let mut n = 0;
        while i < count {
            let instr = (instructions >> ((count - i - 2) * 3)) & 0b111111;
            let (instr, literal) = (instr >> 3, instr & 0b111);
            let combo = move || match literal {
                n @ 0..=3 => n as usize,
                4 => a,
                5 => b,
                6 => c,
                // we just assume we're not gonna use this lol
                n => n as usize,
            };
            match instr {
                // Adv
                0 => a = a / (1 << combo()),
                // Bxl
                1 => b = b ^ (literal as usize),
                // Bst
                2 => b = combo() & 0b111,
                // Jnz
                3 => {
                    if a != 0 {
                        let n = literal as usize;
                        if i != n {
                            i = n;
                            continue;
                        }
                    }
                }
                // Bxc
                4 => {
                    b = b ^ c;
                }
                // Out
                5 => {
                    output = (output << 3) | (combo() as u64 & 0b111);
                    n += 1;
                }
                // Bdv
                6 => b = a / (1 << combo()),
                // Cdv
                7 => c = a / (1 << combo()),
                _ => unreachable!(),
            }

            i += 2;
        }
        (output, n)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let computer: Computer = input.parse().unwrap();
    Some(display(computer.solve()))
}

pub fn part_two(input: &str) -> Option<usize> {
    let computer: Computer = input.parse().unwrap();

    (10usize.pow(14)..10usize.pow(15))
        .into_par_iter()
        .progress_with_style(
            ProgressStyle::with_template("{human_pos}/{len} {wide_bar} {eta_precise}")
                .expect("valid progress bar"),
        )
        .find_first(|a| {
            let mut computer = computer;
            computer.a = *a;
            computer.solve() == computer.instructions
        })
}

fn display((output, n): (u64, usize)) -> String {
    (0..n).rev().map(|i| (output >> i * 3) & 0b111).join(",")
}

impl FromStr for Computer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers, instructions) = s.split_once("\n\n").unwrap();
        let (a, b, c) = registers
            .lines()
            .map(|line| (line.split_once(": ").unwrap().1).parse().unwrap())
            .collect_tuple()
            .unwrap();

        let instructions =
            instructions[9..]
                .trim()
                .split(",")
                .fold((0, 0), |(instructions, count), n| {
                    (
                        (instructions << 3) | (n.parse::<u64>().unwrap() & 0b111),
                        count + 1,
                    )
                });

        println!("{}", display(instructions));

        Ok(Self {
            a,
            b,
            c,
            instructions,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#,
        "4,6,3,5,6,3,5,2,1,0"
    )]
    #[case(
        r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
"#,
        "0,1,2"
    )]
    #[case(
        r#"Register A: 0
Register B: 0
Register C: 9

Program: 2,6,5,5
"#,
        "1"
    )]
    #[case(
        r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0,5,4
"#,
        "4,2,5,6,7,7,7,7,3,1,0,0"
    )]
    #[case(
        r#"Register A: 0
Register B: 29
Register C: 0

Program: 1,7,5,5
"#,
        "2"
    )]
    #[case(
        r#"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0,5,5
"#,
        "2"
    )]
    fn test_part_one(#[case] input: &str, #[case] output: &str) {
        assert_eq!(part_one(input), Some(output.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
