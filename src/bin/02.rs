use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};

advent_of_code::solution!(2);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let line = separated_list1(tag(" "), complete::u32);
    separated_list1(tag("\n"), line)(input)
}

// The levels are either all increasing or all decreasing.
fn is_level_increasing_or_decreasing(xs: &Vec<u32>) -> bool {
    let is_increasing = xs.windows(2).all(|w| w[0] <= w[1]);
    let is_decreasing = xs.windows(2).all(|w| w[0] >= w[1]);
    is_increasing || is_decreasing
}

// Any two adjacent levels differ by at least one and at most three.
fn is_diff_in_range(xs: &Vec<u32>) -> bool {
    xs.windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .all(|diff| diff >= 1 && diff <= 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input).unwrap().1;

    let result = input
        .iter()
        .filter(|report|
            is_level_increasing_or_decreasing(report) && is_diff_in_range(report))
        .count();
    Some(result as u32)
}

fn omits(xs: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut results : Vec<Vec<u32>> = vec![];

    for i in 0..xs.len() {
        let mut r : Vec<u32> = xs.clone();
        r.remove(i);
        results.push(r);
    }
    results.push(xs.clone());

    results
}

fn valid_pt_2(xs: &Vec<u32>) -> bool {
    omits(xs).iter()
        .any(|r| is_level_increasing_or_decreasing(r) && is_diff_in_range(r))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input).unwrap().1;
    let result = input.iter()
        .filter(|report| valid_pt_2(&report))
        .count() as u32;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
