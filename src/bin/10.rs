use advent_of_code::grid::Direction::{E, N, S, W};
use advent_of_code::grid::{Grid, Point};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, map_res};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> IResult<&str, Grid<u8>> {
    let parse_line = many1(map_res(one_of("0123456789"), |c: char| {
        c.to_string().parse::<u8>()
    }));
    map(separated_list1(tag("\n"), parse_line), |data| {
        Grid::new(data)
    })(input)
}

fn neighbors(grid: &Grid<u8>, point: &Point) -> Vec<Point> {
    let tile = grid.get(point).unwrap();

    let ns = [N, E, S, W]
        .iter()
        .map(|d| point.move_direction(d))
        .filter(|p| {
            if let Some(t) = grid.get(p) {
                return t == tile + 1;
            }
            false
        })
        .collect_vec();
    ns
}

fn reachable_ends(grid: &Grid<u8>, start: &Point) -> HashSet<Point> {
    if let Some(9) = grid.get(start) {
        return HashSet::from_iter(vec![*start]);
    }

    neighbors(grid, start)
        .iter()
        .flat_map(|p| reachable_ends(grid, p))
        .collect()
}

fn count_reachable_ends(grid: &Grid<u8>, start: &Point) -> usize {
    if let Some(9) = grid.get(start) {
        return 1;
    }

    neighbors(grid, start)
        .iter()
        .map(|p| count_reachable_ends(grid, p))
        .sum()
}

fn get_start_points(grid: &Grid<u8>) -> Vec<Point> {
    grid.all_points().into_iter()
        .filter(|p| {
            if let Some(0) = grid.get(p) {
                return true;
            }
            false
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input).unwrap().1;
    let starts = get_start_points(&grid);
    let result = starts
        .iter()
        .map(|p| reachable_ends(&grid, p).iter().count())
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input).unwrap().1;
    let starts = get_start_points(&grid);
    let result = starts
        .iter()
        .map(|p| count_reachable_ends(&grid, p))
        .sum();
    Some(result)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
