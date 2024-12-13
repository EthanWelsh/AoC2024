use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::iter::IntoIterator;

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq)]
struct Puzzle {
    total: u64,
    nums: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

fn parse_input(s: &str) -> IResult<&str, Vec<Puzzle>> {
    let parse_line = map(
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(tag(" "), complete::u64),
        ),
        |(total, nums)| Puzzle { total, nums },
    );
    separated_list1(tag("\n"), parse_line)(s)
}

fn can_be_true(ops: &Vec<fn(u64, u64) -> u64>, numbers: &[u64], target: u64) -> bool {
    fn helper(
        ops: &Vec<fn(u64, u64) -> u64>,
        numbers: &[u64],
        target: u64,
        current_value: u64,
        index: usize,
    ) -> bool {
        if index == numbers.len() {
            return current_value == target;
        }

        if index == 0 {
            return helper(ops, numbers, target, numbers[0], index + 1);
        }

        ops.iter().any(|f| {
            helper(ops, numbers, target, f(current_value, numbers[index]), index + 1)
        })
    }

    helper(ops, numbers, target, 0, 0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzles = parse_input(input).unwrap().1;
    let ops: Vec<fn(u64, u64) -> u64> = vec![|a, b| a + b, |a, b| a * b];

    let result = puzzles
        .into_iter()
        .filter(|p| can_be_true(&ops, &p.nums, p.total)) // Pass reference to ops
        .map(|p| p.total)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzles = parse_input(input).unwrap().1;
    let ops: Vec<fn(u64, u64) -> u64> = vec![|a, b| a + b, |a, b| a * b, |a, b| {
        (a.to_string() + &b.to_string()).parse::<u64>().unwrap()
    }];

    let result = puzzles
        .into_iter()
        .filter(|p| can_be_true(&ops, &p.nums, p.total)) // Pass reference to ops
        .map(|p| p.total)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, data) = parse_input(&input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            data.first(),
            Some(&Puzzle {
                total: 190,
                nums: vec![10, 19]
            })
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
