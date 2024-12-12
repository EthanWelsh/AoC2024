use advent_of_code::grid::{Direction, Grid, Point};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::HashSet;

advent_of_code::solution!(6);

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(
        many1(alt((char('.'), char('#'), char('^')))),
        alt((line_ending, eof)),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Grid<char>> {
    map(many1(parse_line), |data| Grid::new(data))(input)
}

fn turn(direction: &Direction) -> Direction {
    match direction {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
        _ => panic!("invalid direction"),
    }
}

fn get_next_point(
    point: &Point,
    direction: &Direction,
    grid: &Grid<char>,
    added_wall: Option<Point>,
) -> (Point, Direction) {
    let forward = point.move_direction(&direction);
    let forward_tile = grid.get(&forward);

    if Some('#') == forward_tile || added_wall == Some(forward) {
        let next_dir = turn(direction);
        return (*point, next_dir);
    }

    (forward, *direction)
}

fn step(point: &Point, dir: &Direction, grid: &Grid<char>) -> Vec<Point> {
    let (next_point, next_dir) = get_next_point(point, dir, grid, None);
    if !grid.in_bounds(&next_point) {
        return vec![*point];
    }
    std::iter::once(*point)
        .chain(step(&next_point, &next_dir, grid))
        .collect_vec()
}

fn get_start_point(grid: &Grid<char>) -> Point {
    grid.all_points()
        .into_iter()
        .find(|p| grid.get(p) == Some('^'))
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input).unwrap().1;
    let start = get_start_point(&grid);
    let points = step(&start, &Direction::N, &grid);
    Some(points.iter().unique().count().try_into().unwrap())
}

fn is_cycle(point: &Point, dir: &Direction, grid: &Grid<char>, added_wall: &Point) -> bool {
    let mut seen_states: HashSet<(Point, Direction)> = HashSet::new();
    let mut current_point = *point;
    let mut current_direction = *dir;
    seen_states.insert((current_point, current_direction));

    loop {
        (current_point, current_direction) =
            get_next_point(&current_point, &current_direction, grid, Some(*added_wall));

        if !grid.in_bounds(&current_point) {
            // The point went out of bounds, so this isn't a cycle.
            return false;
        }

        if seen_states.contains(&(current_point, current_direction)) {
            return true;
        } else {
            seen_states.insert((current_point, current_direction));
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input).unwrap().1;
    let start = get_start_point(&grid);
    let possible_wall_positions = step(&start, &Direction::N, &grid).into_iter().unique().collect_vec();
    let result = possible_wall_positions.iter()
        .filter(|added_wall| is_cycle(&start, &Direction::N, &grid, &added_wall))
        .count();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
