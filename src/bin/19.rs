use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::{count, separated_list1};
use nom::IResult;
use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug, PartialEq, Eq)]
struct Input {
    parts: Vec<String>,
    targets: Vec<String>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, parts) = separated_list1(tag(", "), alpha1)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, targets) = separated_list1(tag("\n"), alpha1)(input)?;

    Ok((
        input,
        Input {
            parts: parts.into_iter().map(String::from).collect(),
            targets: targets.into_iter().map(String::from).collect(),
        },
    ))
}

fn is_possible(target: &[char], parts: &Vec<String>) -> bool {
    fn helper<'a>(
        target: &'a [char],
        parts: &Vec<String>,
        memoize: &mut HashMap<&'a [char], bool>,
    ) -> bool {
        if memoize.contains_key(target) {
            return memoize[target];
        }

        if target.is_empty() {
            return true;
        }

        for part in parts {
            if target.len() >= part.len()
                && target[..part.len()] == part.chars().collect::<Vec<char>>()
            {
                if helper(&target[part.len()..], parts, memoize) {
                    memoize.insert(target, true);
                    return true;
                }
            }
        }
        memoize.insert(target, false);
        false
    }
    helper(target, parts, &mut HashMap::new())
}

pub fn part_one(input: &str) -> Option<u32> {
    let Input { parts, targets } = parse_input(input).unwrap().1;

    let result = targets
        .iter()
        .filter(|target| is_possible(&target.chars().collect::<Vec<char>>(), &parts))
        .count() as u32;

    Some(result)
}

fn ways_to_create(target: &[char], parts: &Vec<String>) -> usize {
    fn helper<'a>(
        target: &'a [char],
        parts: &Vec<String>,
        memoize: &mut HashMap<&'a [char], usize>,
    ) -> usize {
        if memoize.contains_key(target) {
            return memoize[target];
        }

        if target.is_empty() {
            return 1;
        }

        let mut count = 0;
        for part in parts {
            if target.len() >= part.len()
                && target[..part.len()] == part.chars().collect::<Vec<char>>()
            {
                count += helper(&target[part.len()..], parts, memoize);
            }
        }

        memoize.insert(target, count);
        count
    }
    helper(target, parts, &mut HashMap::new())
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { parts, targets } = parse_input(input).unwrap().1;

    let possibles = targets
        .iter()
        .filter(|target| is_possible(&target.chars().collect::<Vec<char>>(), &parts))
        .collect_vec();

    Some(
        possibles
            .iter()
            .map(|p| ways_to_create(&p.chars().collect::<Vec<char>>(), &parts))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, _) = parse_input(&input).unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
