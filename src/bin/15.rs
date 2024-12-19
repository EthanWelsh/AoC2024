use advent_of_code::grid::Direction::{E, N, S, W};
use advent_of_code::grid::{Direction, Grid, Point};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, one_of};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use std::iter;

const SHOULD_PRINT: bool = false;

advent_of_code::solution!(15);

fn parse_grid(input: &str) -> IResult<&str, Grid<char>> {
    map(separated_list1(tag("\n"), many1(one_of("^v<>O.#@"))), |g| {
        Grid::new(g)
    })(input)
}

fn parse_dirs(input: &str) -> IResult<&str, Vec<Direction>> {
    let parse_dir = alt((
        map(char('^'), |_| Some(N)),
        map(char('v'), |_| Some(S)),
        map(char('>'), |_| Some(E)),
        map(char('<'), |_| Some(W)),
        map(line_ending, |_| None), // Ignore line endings
    ));
    map(many1(parse_dir), |dirs| {
        dirs.into_iter().filter_map(|d| d).collect()
    })(input)
}

fn parse_input(input: &str) -> IResult<&str, (Grid<char>, Vec<Direction>)> {
    separated_pair(parse_grid, tag("\n\n"), parse_dirs)(input) // Accept both double line endings
}

// Attempts to perform a move. If the move is possible, it will return a list of one or more swaps
// (in the order that they should be performed). If the move is not possible, returns an empty list.
fn swaps_to_move(grid: &Grid<char>, point: &Point, direction: &Direction) -> Vec<(Point, Point)> {
    let next_point = point.move_direction(direction);
    let next_tile = grid.get(&next_point);

    if next_tile == Some('#') || next_tile == None {
        return vec![];
    }
    if next_tile == Some('.') {
        return vec![(*point, next_point)];
    }
    if next_tile == Some('O') {
        let swaps = swaps_to_move(grid, &next_point, direction);
        return if swaps.is_empty() {
            vec![]
        } else {
            swaps.into_iter().chain(iter::once((*point, next_point))).collect()
        };
    }
    if next_tile == Some('[') || next_tile == Some(']') {
        if direction == &E || direction == &W {
            let swaps = swaps_to_move(grid, &next_point, direction);
            return if swaps.is_empty() {
                vec![]
            } else {
                swaps.into_iter().chain(iter::once((*point, next_point))).collect()
            };
        }

        let left = if next_tile == Some('[') { next_point } else { next_point.move_direction(&W) };
        let right = if next_tile == Some(']') { next_point } else { next_point.move_direction(&E) };


        let left_swaps = swaps_to_move(grid, &left, direction);
        let right_swaps = swaps_to_move(grid, &right, direction);
        if left_swaps.is_empty() || right_swaps.is_empty() {
            return vec![];
        }

        let ret = left_swaps.into_iter()
            .chain(right_swaps)
            .chain(iter::once((*point, next_point))) // Robot swap LAST
            .unique()
            .collect();
        return ret;
    }


    panic!("Invalid next_tile {:?}", grid.get(&next_point));
}

fn perform_swaps(grid: &mut Grid<char>, swaps: &Vec<(Point, Point)>) {
    for swap in swaps {
        let (a, b) = swap;

        let new_a = grid.get(b).unwrap();
        let new_b = grid.get(a).unwrap();
        grid.set(a, new_a);
        grid.set(b, new_b);
    }
}

fn step(grid: &mut Grid<char>, point: &Point, direction: &Direction) -> Point {
    let swaps = swaps_to_move(grid, point, direction);

    if SHOULD_PRINT { println!("Moving {:?}", direction); }
    if swaps.is_empty() {
        if SHOULD_PRINT { println!("{}\n", grid.to_string()); }
        *point
    } else {
        let next_point = point.move_direction(direction);
        perform_swaps(grid, &swaps);
        if SHOULD_PRINT { println!("{}\n", grid.to_string()); }
        next_point
    }
}

fn gps_score(grid: &Grid<char>) -> u64 {
    grid.all_points()
        .iter()
        .filter(|p| grid.get(p) == Some('O') || grid.get(p) == Some('['))
        .map(|p| (p.row * 100 + p.col) as u64)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, directions) = parse_input(input).unwrap().1;
    let mut start = grid.all_points().iter().find(|p| grid.get(p) == Some('@')).unwrap().clone();
    for dir in directions {
        start = step(&mut grid, &start, &dir);
    }
    Some(gps_score(&grid))
}

fn expand(grid: &Grid<char>) -> Grid<char> {
    grid.expand_grid(|c| {
        match c {
            '#' => vec![vec!['#', '#']],
            'O' => vec![vec!['[', ']']],
            '.' => vec![vec!['.', '.']],
            '@' => vec![vec!['@', '.']],
            _  => panic!("Invalid next_tile {:?}", c),
        }
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, directions) = parse_input(input).unwrap().1;
    grid = expand(&grid);

    if SHOULD_PRINT { println!("\n{}\n", grid.to_string()); }

    let mut start = grid.all_points().iter().find(|p| grid.get(p) == Some('@')).unwrap().clone();
    for dir in directions {
        start = step(&mut grid, &start, &dir);
    }
    Some(gps_score(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, _) = parse_input(&input).unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_expand_grid() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (grid, _) = parse_input(input).unwrap().1;
        let expected = indoc! {"
            ####################
            ##....[]....[]..[]##
            ##............[]..##
            ##..[][]....[]..[]##
            ##....[]@.....[]..##
            ##[]##....[]......##
            ##[]....[]....[]..##
            ##..[][]..[]..[][]##
            ##........[]......##
            ####################
        "};
        assert_eq!(expected, expand(&grid).to_string());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
