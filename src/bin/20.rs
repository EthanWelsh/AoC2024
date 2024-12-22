use advent_of_code::grid::Direction::{E, N, S, W};
use advent_of_code::grid::{Grid, Point};
use advent_of_code::search::distance_to_goal;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{line_ending, none_of};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

fn parse_input(input: &str) -> IResult<&str, Grid<char>> {
    let parse_line = terminated(many1(none_of(" \t\n\r")), alt((line_ending, eof)));
    map(many1(parse_line), |data| Grid::new(data))(input)
}

fn neighbors(grid: &Grid<char>, current: &Point) -> Vec<(Point, u64)> {
    [N, S, E, W]
        .into_iter()
        .map(|d| current.move_direction(&d))
        .filter(|point| grid.in_bounds(point))
        .filter(|point| grid.get(point) != Some('#'))
        .map(|point| (point, 1))
        .collect_vec()
}

fn get_distances_to_goal(grid: &Grid<char>) -> HashMap<Point, u64> {
    let end = grid.find(|c| c == 'E').unwrap();
    let nebs = |point: &Point| neighbors(&grid, point);
    distance_to_goal(&end, &nebs)
}

fn count_cheats(g: &Grid<char>, cheat_distance: u32, count_cheats_higher_than: u32) -> u64 {
    let distances = get_distances_to_goal(&g);

    // Remove the start and end tiles so that we don't need to deal with them.
    let mut grid = g.clone();
    let start = grid.find(|c| c == 'S').unwrap();
    let end = grid.find(|c| c == 'E').unwrap();
    grid.set(&start, '.');
    grid.set(&end, '.');

    let free_spots = grid
        .all_points()
        .into_iter()
        .filter(|point| grid.get(point) != Some('#'))
        .collect_vec();

    let mut cheats : HashSet<(Point, Point)> = HashSet::new();
    for entrance in &free_spots {
        let exits = free_spots.iter().filter(|point| {
            entrance.manhattan_distance(point) <= cheat_distance
        }).collect_vec();

        exits.into_iter().for_each(|exit| {
            let entrance_distance = *distances.get(&entrance).unwrap() as i64;
            let exit_distance = *distances.get(&exit).unwrap() as i64;
            let cheat_length = entrance.manhattan_distance(exit) as i64;

            let saved_distance = entrance_distance - exit_distance - (cheat_length - 1);

            if saved_distance >= count_cheats_higher_than as i64 {
                if !cheats.contains(&(*entrance, *exit)) && !cheats.contains(&(*exit, *entrance)) {
                    cheats.insert((*entrance, *exit));
                }
            }
        })
    }
    cheats.len() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input).unwrap().1;
    Some(count_cheats(&grid, 2, 100))
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input).unwrap().1;
    Some(count_cheats(&grid, 20, 100))
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
}
