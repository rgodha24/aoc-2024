advent_of_code::solution!(24);
use std::collections::HashMap;

use advent_of_code::helpers::*;
use itertools::Itertools;

macro_rules! name {
    ($c:literal, $n: expr) => {
        format!("{}{:02}", $c, $n).as_str()
    };
}

#[derive(Debug, Clone, Eq)]
enum Register<'a> {
    Const(bool),
    Op(&'a str, Op, &'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    And,
    Xor,
    Or,
}

impl PartialEq for Register<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Register::Const(l), Register::Const(r)) => l == r,
            (Register::Op(l1, lop, l2), Register::Op(r1, rop, r2)) => {
                // a ^ b == b ^ a, a & b == b & a, a | b == b | a, so switching the direction still
                // means equal registers
                lop == rop && ((l1 == r1 && l2 == r2) | (l1 == r2 && l2 == r1))
            }
            _ => false,
        }
    }
}

impl Register<'_> {
    fn value<'a>(&self, registers: &HashMap<&'a str, Register>) -> bool {
        match &self {
            Register::Const(b) => *b,
            Register::Op(r1, op, r2) => {
                let r1 = registers.get(r1).cloned().unwrap().value(registers);
                let r2 = registers.get(r2).cloned().unwrap().value(registers);

                match op {
                    Op::And => r1 & r2,
                    Op::Xor => r1 ^ r2,
                    Op::Or => r1 | r2,
                }
            }
        }
    }
}

fn parse(input: &str) -> (HashMap<&str, Register>, usize) {
    let (values, ops) = input.split_once("\n\n").unwrap();
    let mut registers = HashMap::new();
    for v in values.lines() {
        let (register, b) = v.split_once(": ").unwrap();
        registers.insert(register, Register::Const(b == "1"));
    }

    for o in ops.trim().lines() {
        let (r1, op, r2, _, output) = o.split_whitespace().collect_tuple().unwrap();
        registers.insert(
            output,
            Register::Op(
                r1,
                match op {
                    "AND" => Op::And,
                    "XOR" => Op::Xor,
                    "OR" => Op::Or,
                    s => panic!("unknown op {s}"),
                },
                r2,
            ),
        );
    }
    let nbits = (0..100)
        .take_while(|n| registers.contains_key(name!('z', n)))
        .last()
        .unwrap();

    (registers, nbits)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (registers, nbits) = parse(input);

    let mut answer = 0u64;
    for n in 0..=nbits {
        let r = registers[name!('z', n)].clone();
        answer |= (if r.value(&registers) { 1 } else { 0 }) << n;
    }

    Some(answer)
}

/// basically, we're trying to create the correct implementation of a full adder
/// a full adder is a function like this, using the terminology of the problem,
/// where `c` is a variable for the "carried" bit
/// (z_n, c_{n+1}) = add(x_n, y_n, c_n)
/// where z and c_{n+1} are
/// z = c_n ^ (x_n ^ y_n)
/// c_{n+1} = (x_n & y_n) | (c_n & x_n ^ y_n)
///
/// we can assign these bit combinations variable names
/// a := x_n ^ y_n, b := x_n & y_n, d := c_n & a
///
/// and set z_n := c_n ^ a, and c_{n+1} := b | d
///
/// if we store the registers we expect to find for the zs and cs, we can find the incorrect gates.
pub fn part_two(input: &str) -> Option<String> {
    let (registers, nbits) = parse(input);

    // -1 because z_{nbits} == c_{nbits}
    // let a_list = (0..=nbits)
    //     .map(|n| {
    //         registers
    //             .iter()
    //             .find(|&(_, v)| Register::Op(name!('x', n), Op::Xor, name!('y', n)) == *v)
    //             .map(|(r, _)| r)
    //             .unwrap_or(&"$$$")
    //     })
    //     .collect_vec();
    //
    // let b_list = (0..=nbits - 1)
    //     .map(|n| {
    //         registers
    //             .iter()
    //             .find(|&(_, v)| Register::Op(name!('x', n), Op::And, name!('y', n)) == *v)
    //             .unwrap()
    //             .0
    //     })
    //     .collect_vec();
    //
    // // starting at 1 because c_0 does not exist
    // let mut c_list = vec!["$$$"];
    // let mut d_list = vec![];
    //
    // for n in 0..=nbits - 1 {
    //     d_list.push(
    //         registers
    //             .iter()
    //             .find(|&(_, v)| Register::Op(c_list[n], Op::And, a_list[n]) == *v)
    //             .map(|(r, _)| r)
    //             .unwrap_or(a_list[n]),
    //     );
    //
    // c_list.push(
    //     registers
    //         .iter()
    //         .find(|&(_, v)| Register::Op(b_list[n], Op::Or, d_list[n]) == *v)
    //         .map(|(r, _)| r)
    //         .unwrap_or(b_list[n]),
    // );
    // }
    //
    // let z_list = (0..=nbits)
    //     .map(|n| {
    //         registers
    //             .iter()
    //             .find(|&(_, v)| Register::Op(c_list[n], Op::Xor, a_list[n]) == *v)
    //             .map(|(r, _)| r)
    //             .unwrap_or(a_list[n])
    //     })
    //     .collect_vec();

    let mut n = 0;
    let mut c = &"$$$";

    while n <= nbits {
        let a = registers
            .iter()
            .find(|&(_, v)| Register::Op(name!('x', n), Op::Xor, name!('y', n)) == *v)
            .map(|(r, _)| r)
            .unwrap_or(&"$$$");
        let b = registers
            .iter()
            .find(|&(_, v)| Register::Op(name!('x', n), Op::And, name!('y', n)) == *v)
            .map(|(r, _)| r)
            .unwrap_or(&"$$$");

        let d = registers
            .iter()
            .find(|&(_, v)| Register::Op(c, Op::And, a) == *v)
            .map(|(r, _)| r)
            .unwrap_or(a);

        let next_c = registers
            .iter()
            .find(|&(_, v)| Register::Op(b, Op::Or, d) == *v)
            .map(|(r, _)| r)
            .unwrap_or(b);

        let z = *registers
            .iter()
            .find(|&(_, v)| Register::Op(c, Op::Xor, a) == *v)
            .map(|(r, _)| r)
            .unwrap_or(a);

        let expected_z = name!('z', n).to_string();

        if z == expected_z {
            n += 1;
            c = next_c;
            continue;
        }

        println!("mismatching z {n}");
        dbg!(a, b, c, d, z, expected_z);
        break;
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#,
        0b100
    )]
    #[case(&advent_of_code::template::read_file("examples", DAY), 0b0011111101000)]
    fn test_part_one(#[case] input: &str, #[case] output: u64) {
        let result = part_one(input);
        assert_eq!(result, Some(output));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
