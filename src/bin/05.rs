use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

#[derive(Debug, PartialEq, Eq)]
struct Puzzle {
    rules: HashMap<i64, HashSet<i64>>,
    updates: Vec<Vec<i64>>,
}

fn parse_input(input: &str) -> IResult<&str, Puzzle> {
    let pair = separated_pair(complete::i64, tag("|"), complete::i64);
    let pairs = map(separated_list1(tag("\n"), pair), |ps| {
        let mut rule_map: HashMap<i64, HashSet<i64>> = HashMap::new();
        for p in ps {
            rule_map.entry(p.0).or_default().insert(p.1);
        }
        rule_map
    });
    let update = separated_list1(tag(","), complete::i64);
    let updates = separated_list1(tag("\n"), update);

    map(separated_pair(pairs, tag("\n\n"), updates), |(rs, us)| {
        Puzzle {
            rules: rs,
            updates: us,
        }
    })(input)
}

fn violations(update: &Vec<i64>, rules: &HashMap<i64, HashSet<i64>>) -> Vec<(i64, i64)> {
    let mut seen_so_far = HashSet::new();
    let mut violations = Vec::new();

    for u in update.iter() {
        if let Some(should_appear_after) = rules.get(u) {
            for after in should_appear_after {
                if seen_so_far.contains(after) {
                    violations.push((*u, *after));
                }
            }
        }
        seen_so_far.insert(u);
    }
    violations
}

fn is_allowed(update: &Vec<i64>, rules: &HashMap<i64, HashSet<i64>>) -> bool {
    violations(update, rules).is_empty()
}

fn get_middle(update: &Vec<i64>) -> i64 {
    let middle_index = update.len() / 2;
    update[middle_index]
}

pub fn part_one(input: &str) -> Option<i64> {
    let puzzle = parse_input(input).unwrap().1;
    let result = puzzle
        .updates
        .iter()
        .filter(|u| is_allowed(u, &puzzle.rules))
        .map(|u| get_middle(u))
        .sum();
    Some(result)
}

fn swap(update: &Vec<i64>, a: i64, b: i64) -> Option<Vec<i64>> {
    let mut a_index = None;
    let mut b_index = None;
    for (i, &x) in update.iter().enumerate() {
        if x == a {
            a_index = Some(i);
        }
        if x == b {
            b_index = Some(i);
        }
        if a_index.is_some() && b_index.is_some() {
            break;
        }
    }
    if let (Some(a_index), Some(b_index)) = (a_index, b_index) {
        let mut result = update.clone();
        result.swap(a_index, b_index);
        Some(result)
    } else {
        None
    }
}

fn fix_update(update: &Vec<i64>, rules: &HashMap<i64, HashSet<i64>>) -> Vec<i64> {
    let mut result = update.clone();
    result.sort_by(|a, b| {
        if rules.get(a).unwrap().contains(b) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    result
}

pub fn part_two(input: &str) -> Option<i64> {
    let puzzle = parse_input(input).unwrap().1;
    let result = puzzle
        .updates
        .iter()
        .filter(|u| !is_allowed(u, &puzzle.rules))
        .map(|update| fix_update(update, &puzzle.rules))
        .map(|update| get_middle(&update))
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
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }
}
