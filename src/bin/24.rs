advent_of_code::solution!(24);
use std::collections::{HashMap, HashSet};

use advent_of_code::helpers::*;
use indicatif::*;
use itertools::Itertools;
use num::BigInt;
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Register<'a> {
    Const(bool),
    Op(&'a str, Op, &'a str),
}
#[derive(Debug, Clone)]
enum Op {
    And,
    Xor,
    Or,
}

impl Register<'_> {
    fn value<'a>(
        &self,
        registers: &HashMap<&'a str, Register>,
        visited: &mut Option<&'a mut HashSet<String>>,
    ) -> Option<bool> {
        Some(match self {
            Register::Const(b) => *b,
            Register::Op(r1, op, r2) => {
                if visited.as_mut().is_some_and(|visited| {
                    visited.insert(r1.to_string()) || visited.insert(r2.to_string())
                }) {
                    return None;
                }

                let r1 = registers
                    .get(r1)
                    .cloned()
                    .unwrap()
                    .value(registers, visited)?;
                let r2 = registers
                    .get(r2)
                    .cloned()
                    .unwrap()
                    .value(registers, visited)?;

                match op {
                    Op::And => r1 & r2,
                    Op::Xor => r1 ^ r2,
                    Op::Or => r1 | r2,
                }
            }
        })
    }
    fn children<'a>(&self, registers: &'a HashMap<&'a str, Register>) -> Vec<String> {
        match self {
            Register::Const(_) => vec![],
            Register::Op(r1, _, r2) => {
                let mut v = vec![r1.to_string(), r2.to_string()];
                v.extend_from_slice(&registers[r1].children(registers));
                v.extend_from_slice(&registers[r2].children(registers));
                v
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (values, ops) = input.split_once("\n\n").unwrap();
    let mut registers: HashMap<&str, Register> = HashMap::new();
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

    let mut answer = 0u64;
    for z in 0..99 {
        if let Some(r) = registers.get(format!("z{z:02}").as_str()).cloned() {
            answer += (if r.value(&registers, &mut None).unwrap() {
                1
            } else {
                0
            }) << z;
        } else {
            break;
        }
    }

    dbg!(registers
        .values()
        .filter(|r| matches!(r, Register::Op(..)))
        .collect_vec()
        .len());

    Some(answer)
}

pub fn part_two(input: &str) -> Option<String> {
    let (values, ops) = input.split_once("\n\n").unwrap();
    let mut registers: HashMap<&str, Register> = HashMap::new();
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

    // no longer mutable
    let registers = registers;

    let x: u64 = (0..64)
        .fold_while(0, |acc, n| {
            use itertools::FoldWhile::{Continue, Done};
            if let Some(r) = registers.get(format!("x{n:02}").as_str()).cloned() {
                Continue(
                    acc + ((if r.value(&registers, &mut None).unwrap() {
                        1
                    } else {
                        0
                    }) << n),
                )
            } else {
                Done(acc)
            }
        })
        .into_inner();
    let y: u64 = (0..64)
        .fold_while(0, |acc, n| {
            use itertools::FoldWhile::{Continue, Done};
            if let Some(r) = registers.get(format!("y{n:02}").as_str()).cloned() {
                Continue(
                    acc + ((if r.value(&registers, &mut None).unwrap() {
                        1
                    } else {
                        0
                    }) << n),
                )
            } else {
                Done(acc)
            }
        })
        .into_inner();

    let expected_z = x + y;
    let curr_z: u64 = (0..64)
        .fold_while(0, |acc, n| {
            use itertools::FoldWhile::{Continue, Done};
            if let Some(r) = registers.get(format!("z{n:02}").as_str()).cloned() {
                Continue(
                    acc + ((if r.value(&registers, &mut None).unwrap() {
                        1
                    } else {
                        0
                    }) << n),
                )
            } else {
                Done(acc)
            }
        })
        .into_inner();

    dbg!(expected_z, curr_z);
    println!("exp {expected_z:b}");
    println!("cur {curr_z:b}");

    let (curr_zero, curr_one): (Vec<_>, Vec<_>) = (0..64)
        .filter(|n| (expected_z ^ curr_z) & (0b1 << n) > 0)
        .flat_map(|z| {
            // let mut v = registers
            //     .get(format!("z{z:02}").as_str())
            //     .unwrap()
            //     .children(&registers);
            // v.push(format!("z{z:02}"));
            // v
            [format!("z{z:02}")]
        })
        .unique()
        .partition(|reg| {
            !registers
                .get(reg.as_str())
                .unwrap()
                .value(&registers, &mut None)
                .unwrap()
        });

    dbg!(&curr_zero);
    dbg!(&curr_one);

    let n = curr_zero.len() as u64;

    curr_zero
        .into_iter()
        .tuple_combinations()
        .progress_count((n) * (n - 1) * (n - 2) * (n - 3))
        .find_map(|(s1, s3, s5, s7)| {
            curr_one
                .iter()
                .tuple_combinations()
                .find_map(|(s2, s4, s6, s8)| {
                    let mut registers = registers.clone();
                    macro_rules! swap {
                        ($a:expr, $b:expr) => {
                            let a = registers.get_mut($a.to_string().as_str()).unwrap() as *mut _;
                            let b = registers.get_mut($b.to_string().as_str()).unwrap() as *mut _;
                            unsafe { std::ptr::swap(a, b) };
                        };
                    }
                    // println!("swapping {s1} {s2} {s3} {s4} {s5} {s6} {s7} {s8}");
                    swap!(s1, s2);
                    swap!(s3, s4);
                    swap!(s5, s6);
                    swap!(s7, s8);

                    let z = (0..64)
                        .fold_while(Some(0), |acc, n| {
                            use itertools::FoldWhile::{Continue, Done};
                            if let Some(r) = registers.get(format!("z{n:02}").as_str()).cloned() {
                                let mut visited = HashSet::new();
                                let val = match r.value(&registers, &mut Some(&mut visited)) {
                                    Some(true) => 1,
                                    Some(false) => 0,
                                    None => {
                                        return Done(None);
                                    }
                                };
                                Continue(Some(acc.expect("cant be none") + (val << n)))
                            } else {
                                Done(acc)
                            }
                        })
                        .into_inner()?;

                    (z == expected_z).then_some(
                        [&s1, s2, &s3, s4, &s5, s6, &s7, s8]
                            .into_iter()
                            .sorted()
                            .join(","),
                    )
                })
        })
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
