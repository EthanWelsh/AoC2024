advent_of_code::solution!(11);

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};
use std::collections::HashMap;

fn parse_input(s: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), complete::u64)(s)
}

fn has_even_digit_count(stone: &u64) -> bool {
    stone.to_string().len() % 2 == 0
}

fn split_stone(stone: &u64) -> Vec<u64> {
    let s = stone.to_string();
    let parts = s.split_at(s.len() / 2);
    vec![
        parts.0.parse::<u64>().unwrap(),
        parts.1.parse::<u64>().unwrap(),
    ]
}

fn blink(stone: u64, count: u64, new_stones: &mut HashMap<u64, u64>) {
    if stone == 0 {
        *new_stones.entry(1).or_insert(0) += count;
    } else if has_even_digit_count(&stone) {
        let split = split_stone(&stone);
        *new_stones.entry(split[0]).or_insert(0) += count;
        *new_stones.entry(split[1]).or_insert(0) += count;
    } else {
        *new_stones.entry(stone * 2024).or_insert(0) += count;
    }
}

fn step(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    for (&stone, &count) in stones {
        blink(stone, count, &mut new_stones);
    }
    new_stones
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse_input(input).unwrap().1;
    let mut stones: HashMap<u64, u64> = input.into_iter().map(|x| (x, 1)).collect();
    for _ in 0..25 {
        stones = step(&stones);
    }
    Some(stones.values().sum::<u64>() as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse_input(input).unwrap().1;
    let mut stones: HashMap<u64, u64> = input.into_iter().map(|x| (x, 1)).collect();
    for _ in 0..75 {
        stones = step(&stones);
    }
    Some(stones.values().sum::<u64>() as usize)
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
        assert_eq!(result, Some(55312));
    }
}
