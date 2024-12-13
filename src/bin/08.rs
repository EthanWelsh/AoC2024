use advent_of_code::grid::{Grid, Point};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{line_ending, none_of};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::iter::successors;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> IResult<&str, Grid<char>> {
    let parse_line = terminated(many1(none_of(" \t\n\r")), alt((line_ending, eof)));
    map(many1(parse_line), |data| Grid::new(data))(input)
}

fn compute_nodes(
    grid: &Grid<char>,
    points: &HashSet<Point>,
    should_spread_signal: bool,
) -> HashSet<Point> {
    let pairs: Vec<(Point, Point)> = points
        .iter()
        .combinations(2)
        .map(|ps| (*ps[0], *ps[1]))
        .collect();
    pairs
        .iter()
        .flat_map(|(a, b)| {
            if should_spread_signal {
                spread_signal(grid, a, b)
            } else {
                let difference = a.subtract(b);
                HashSet::from([a.add(&difference), b.subtract(&difference)])
            }
        })
        .filter(|p| grid.in_bounds(p))
        .collect()
}

fn spread_signal(grid: &Grid<char>, a: &Point, b: &Point) -> HashSet<Point> {
    let difference = a.subtract(b);

    let lower: HashSet<Point> = successors(Some(a.clone()), |p| Some(p.add(&difference)))
        .take_while(|p| grid.in_bounds(p))
        .collect();

    let upper: HashSet<Point> = successors(Some(b.clone()), |p| Some(p.subtract(&difference)))
        .take_while(|p| grid.in_bounds(p))
        .collect();

    lower.union(&upper).cloned().collect()
}

fn get_letters_map(grid: &Grid<char>) -> HashMap<char, HashSet<Point>> {
    let letters: HashMap<char, HashSet<Point>> = grid
        .all_points()
        .iter()
        .filter_map(|p| grid.get(p).map(|c| (c, *p)))
        .filter(|(c, _)| *c != '.')
        .sorted()
        .chunk_by(|(c, _)| *c)
        .into_iter()
        .map(|(key, group)| (key, group.map(|(_, p)| p).collect()))
        .collect();
    letters
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input).unwrap().1;
    let letters = get_letters_map(&grid);
    let nodes: HashSet<Point> = letters
        .values()
        .flat_map(|ps| compute_nodes(&grid, ps, false))
        .collect();

    Some(nodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input).unwrap().1;
    let letters = get_letters_map(&grid);
    let nodes: HashSet<Point> = letters
        .values()
        .flat_map(|ps| compute_nodes(&grid, ps, true))
        .collect();

    Some(nodes.len())
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
