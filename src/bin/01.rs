advent_of_code::solution!(1);

use itertools::Itertools;
use nom::character::complete::multispace1;
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    IResult,
};
use num_traits::abs;

fn parse_input(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let line_parser = separated_pair(complete::i32, multispace1, complete::i32);
    separated_list1(tag("\n"), line_parser)(s)
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse_input(input).unwrap().1;

    let mut left = input.iter().map(|(x, _)| x).collect::<Vec<_>>();
    let mut right = input.iter().map(|(_, y)| y).collect::<Vec<_>>();
    left.sort_unstable();
    right.sort_unstable();

    let result = left.iter()
        .zip(right.iter())
        .map(|(x, y)| abs(*x - *y)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse_input(input).unwrap().1;
    let left = input.iter().map(|(x, _)| x).collect::<Vec<_>>();
    let right = input.iter().map(|(_, y)| y).collect::<Vec<_>>();

    let counts = right.iter().counts();

    let result = left.iter()
        .map(|x| *x * (*counts.get(x).unwrap_or(&0) as i32))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, data) = parse_input(&input).unwrap();

        assert_eq!("", remaining);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
