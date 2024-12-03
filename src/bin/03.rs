advent_of_code::solution!(3);
use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
num = { ('0'..'9')+ }

mul  = { "mul(" ~ num ~ "," ~ num ~ ")" }
do   = { "do()" }
dont = { "don't()" }

input = { (mul | do | dont | ANY)+ ~ EOI }
"#]
struct Day3Parser;

fn solve(input: &str, ignore_conditionals: bool) -> i64 {
    let mut enabled = true;
    let mut sum = 0;
    let input_rule = Day3Parser::parse(Rule::input, input)
        .unwrap()
        .next()
        .expect("input rule first");

    for instruction in input_rule.into_inner() {
        match instruction.as_rule() {
            Rule::mul => {
                if ignore_conditionals || enabled {
                    let (n1, n2) = instruction
                        .into_inner()
                        .map(|r| r.as_span().as_str().parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    sum += n1 * n2;
                }
            }
            Rule::r#do => enabled = true,
            Rule::dont => enabled = false,
            _ => break,
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, true))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
