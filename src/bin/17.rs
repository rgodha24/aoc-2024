advent_of_code::solution!(17);

use itertools::Itertools;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Computer {
    instructions: Instruction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction(usize, usize);
impl Instruction {
    #[inline]
    fn at(&self, index: usize) -> (usize, usize) {
        let instr = (self.0 >> ((self.1 - index - 2) * 3)) & 0b111111;
        (instr >> 3, instr & 0b111)
    }
    fn compile(self) -> Computer {
        // TODO: jit compile this?
        Computer { instructions: self }
    }
}

impl Computer {
    fn solve(&self, mut a: usize, mut b: usize, mut c: usize) -> Instruction {
        let Computer { instructions } = self;

        let mut i = 0;
        let mut output = Instruction(0, 0);
        while i < instructions.1 {
            let (instr, literal) = instructions.at(i);
            let combo = match literal {
                n @ 0..=3 => n,
                4 => a,
                5 => b,
                6 => c,
                // we just assume we're not gonna use this lol
                n => n,
            };

            i += 2;

            match instr {
                // Adv
                0 => a /= 1 << combo,
                // Bxl
                1 => b ^= literal,
                // Bst
                2 => b = combo & 0b111,
                // Jnz
                3 => {
                    if a != 0 && i != literal {
                        i = literal;
                    }
                }
                // Bxc
                4 => {
                    b ^= c;
                }
                // Out
                5 => {
                    output.0 = (output.0 << 3) | (combo & 0b111);
                    output.1 += 1;
                }
                // Bdv
                6 => b = a / (1 << combo),
                // Cdv
                7 => c = a / (1 << combo),
                _ => unreachable!(),
            }
        }

        output
    }
}

pub fn part_one(input: &str) -> Option<Instruction> {
    let (a, b, c, instructions) = parse(input);
    let computer = instructions.compile();
    Some(computer.solve(a, b, c))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, b, c, instructions) = parse(input);
    let computer = instructions.compile();

    find_a(computer, 0, b, c, 0)
}

fn find_a(computer: Computer, curr_a: usize, b: usize, c: usize, depth: usize) -> Option<usize> {
    // b == 0 and c == 0 in the input but might as well just pass it in
    let output = computer.solve(curr_a, b, c);

    if output == computer.instructions {
        return Some(curr_a);
    }

    // this if condition is making sure the last `depth * 3` bits
    // of the instructions equals our current output
    if (output.0 ^ computer.instructions.0) & ((1 << (depth * 3)) - 1) == 0 {
        (0b000..=0b111)
            // dfs lower on all of the options for the next 3 bits
            .filter_map(|a| find_a(computer, (curr_a << 3) | a, b, c, depth + 1))
            // and choose the lowest a value if there is one
            .min()
    } else {
        None
    }
}

fn parse(s: &str) -> (usize, usize, usize, Instruction) {
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
            .fold(Instruction(0, 0), |instruction, n| {
                Instruction(
                    (instruction.0 << 3) | (n.parse::<usize>().unwrap() & 0b111),
                    instruction.1 + 1,
                )
            });

    (a, b, c, instructions)
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (output, n) = (self.0, self.1);
        write!(
            f,
            "{}",
            (0..n).rev().map(|i| (output >> (i * 3)) & 0b111).join(",")
        )
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
        assert_eq!(
            part_one(input).map(|instr| instr.to_string()),
            Some(output.to_string())
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
