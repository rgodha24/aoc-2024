advent_of_code::solution!(24);
use std::collections::HashMap;

use advent_of_code::helpers::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Register<'a> {
    kind: RegisterKind<'a>,
    name: &'a str,
}

macro_rules! name {
    ($c:literal, $n: expr) => {
        format!("{}{:02}", $c, $n).as_str()
    };
}

#[derive(Debug, Clone, Eq)]
enum RegisterKind<'a> {
    Const(bool),
    Op(&'a str, Op, &'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    And,
    Xor,
    Or,
}

impl PartialEq for RegisterKind<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RegisterKind::Const(l), RegisterKind::Const(r)) => l == r,
            (RegisterKind::Op(l1, lop, l2), RegisterKind::Op(r1, rop, r2)) => {
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
        match &self.kind {
            RegisterKind::Const(b) => *b,
            RegisterKind::Op(r1, op, r2) => {
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
        registers.insert(
            register,
            Register {
                kind: RegisterKind::Const(b == "1"),
                name: register,
            },
        );
    }

    for o in ops.trim().lines() {
        let (r1, op, r2, _, output) = o.split_whitespace().collect_tuple().unwrap();
        registers.insert(
            output,
            Register {
                kind: RegisterKind::Op(
                    r1,
                    match op {
                        "AND" => Op::And,
                        "XOR" => Op::Xor,
                        "OR" => Op::Or,
                        s => panic!("unknown op {s}"),
                    },
                    r2,
                ),
                name: output,
            },
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
    let registers = parse(input);

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
