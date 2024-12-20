use advent_of_code::grid::Direction::{E, N, S, W};
use advent_of_code::grid::{Direction, Grid, Point};
use advent_of_code::search::{dijkstra, dijkstra_all_shortest_paths};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{line_ending, none_of};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

advent_of_code::solution!(16);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    point: Point,
    heading: Direction,
}

fn parse_input(input: &str) -> IResult<&str, Grid<char>> {
    let parse_line = terminated(many1(none_of(" \t\n\r")), alt((line_ending, eof)));
    map(many1(parse_line), |data| Grid::new(data))(input)
}

fn neighbors(grid: &Grid<char>, current: &State) -> Vec<(State, u64)> {
    let mut neighbors = Vec::new();

    let forward_point = current.point.move_direction(&current.heading);
    if grid.get(&forward_point) != Some('#') {
        neighbors.push((
            State {
                point: forward_point,
                heading: current.heading,
            },
            1,
        ));
    }

    let turn_directions = match current.heading {
        N | S => vec![E, W],
        E | W => vec![N, S],
        _ => panic!("Unexpected heading direction"),
    };

    for &d in &turn_directions {
        neighbors.push((
            State {
                point: current.point,
                heading: d,
            },
            1000,
        ));
    }

    neighbors
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input).unwrap().1;
    let start = grid.find(|c| c == 'S').unwrap();
    let end = grid.find(|c| c == 'E').unwrap();

    let start_state = State {
        point: start,
        heading: E,
    };
    let is_goal = |state: &State| state.point == end;
    let nebs = |state: &State| neighbors(&grid, state);

    let (_, cost) = dijkstra(&start_state, &nebs, &is_goal).unwrap();

    Some(cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input).unwrap().1;
    let start = grid.find(|c| c == 'S').unwrap();
    let end = grid.find(|c| c == 'E').unwrap();

    let start_state = State {
        point: start,
        heading: E,
    };
    let is_goal = |state: &State| state.point == end;
    let nebs = |state: &State| neighbors(&grid, state);
    let (paths, _) = dijkstra_all_shortest_paths(&start_state, &nebs, &is_goal).unwrap();
    let result = paths
        .iter()
        .flatten()
        .map(|state| state.point)
        .unique()
        .count() as u32;

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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
