use crate::Instruction::{Do, Dont, Mult};
use nom::branch::alt;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::{bytes::complete::tag, character::complete, IResult};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Eq)]
struct Multiply {
    a: u32,
    b: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Do,
    Dont,
    Mult(Multiply),
}

fn parse_multiply(input: &str) -> IResult<&str, Multiply> {
    map(
        delimited(
            tag("mul("),
            separated_pair(complete::u32, tag(","), complete::u32),
            tag(")"),
        ),
        |((a, b))| Multiply { a, b },
    )(input)
}

fn parse_pt1(s: &str) -> IResult<&str, Vec<Multiply>> {
    many1(map(many_till(anychar, parse_multiply), |(_, res)| res))(s)
}

pub fn part_one(input: &str) -> Option<u32> {
    let ms = parse_pt1(input).unwrap().1;
    let result = ms.into_iter().map(|m| m.a * m.b).sum();
    Some(result)
}

fn parse_pt2(s: &str) -> IResult<&str, Vec<Instruction>> {
    let parse_do = map(tag("do()"), |_| Do);
    let parse_dont = map(tag("don't()"), |_| Dont);
    let parse_mult = map(parse_multiply, |m| Mult(m));
    let parse_instruction = alt((parse_mult, parse_do, parse_dont));

    many1(map(many_till(anychar, parse_instruction), |(_, res)| res))(s)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_pt2(input).unwrap().1;

    let mut enabled = true;
    let mut total = 0;
    for instruction in instructions {
        if let Do = instruction {
            enabled = true;
        } else if let Dont = instruction {
            enabled = false;
        } else if let Mult(Multiply { a, b }) = instruction {
            if enabled {
                total += a * b;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pt1() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, data) = parse_pt1(&input).unwrap();

        assert_eq!(
            data,
            vec![
                Multiply { a: 2, b: 4 },
                Multiply { a: 5, b: 5 },
                Multiply { a: 11, b: 8 },
                Multiply { a: 8, b: 5 }
            ]
        );
    }

    #[test]
    fn test_parse_pt2() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, data) = parse_pt2(&input).unwrap();

        assert_eq!(
            data,
            vec![
                Mult(Multiply { a: 2, b: 4 }),
                Dont,
                Mult(Multiply { a: 5, b: 5 }),
                Mult(Multiply { a: 11, b: 8 }),
                Do,
                Mult(Multiply { a: 8, b: 5 })
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
