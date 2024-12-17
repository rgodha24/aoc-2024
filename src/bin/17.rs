advent_of_code::solution!(17);
use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

pub fn part_one(input: &str) -> Option<String> {
    let (registers, instructions) = input.split_once("\n\n").unwrap();
    let (mut a, mut b, mut c) = registers
        .lines()
        .map(|line| (line.split_once(": ").unwrap().1).parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let instructions: Vec<(Instruction, usize)> = instructions[9..]
        .trim()
        .split(",")
        .tuples()
        .map(|(instr, n)| {
            use Instruction::*;
            (
                [Adv, Bxl, Bst, Jnz, Bxc, Out, Bdv, Cdv][(instr).parse::<usize>().unwrap()],
                n.parse().unwrap(),
            )
        })
        .collect_vec();

    let mut i = 0;
    let mut output = Vec::new();
    while i < instructions.len() {
        let (instruction, literal) = instructions[i].clone();
        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => usize::MAX,
        };
        match instruction {
            Instruction::Adv => a = a / (1 << combo),
            Instruction::Bxl => b = b ^ literal,
            Instruction::Bst => b = combo & 0b111,
            Instruction::Jnz => {
                if a != 0 {
                    assert!(literal % 2 == 0);
                    if i != literal as usize / 2 {
                        i = literal as usize / 2;
                        continue;
                    }
                }
            }
            Instruction::Bxc => {
                b = b ^ c;
            }
            Instruction::Out => {
                output.push(combo & 0b111);
            }
            Instruction::Bdv => b = a / (1 << combo),
            Instruction::Cdv => c = a / (1 << combo),
        }

        i += 1;
    }

    Some(output.into_iter().join(","))
}

pub fn part_two(input: &str) -> Option<i64> {
    None
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
        assert_eq!(result, None);
    }
}
