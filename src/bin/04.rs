use advent_of_code::grid::Direction::{E, N, NE, NW, S, SE, SW, W};
use advent_of_code::grid::{Grid, Point};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

advent_of_code::solution!(4);

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(
        many1(alt((char('X'), char('M'), char('A'), char('S')))),
        alt((line_ending, eof)),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Grid<char>> {
    map(many1(parse_line), |data| Grid::new(data))(input)
}

fn all_points_from(point: &Point) -> Vec<Vec<Point>> {
    let dirs = vec![N, NE, E, SE, S, SW, W, NW];

    dirs.iter()
        .map(|d| itertools::repeat_n(*d, 3).collect::<Vec<_>>())
        .map(|ds| point.move_directions(&ds))
        .collect::<Vec<_>>()
}

fn strings_from_points(grid: &Grid<char>, points: Vec<Vec<Point>>) -> Vec<String> {
    points
        .iter()
        .map(|ps| {
            ps.iter().map(|p| grid.get(p).unwrap_or(' ')).collect::<String>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input).unwrap().1;
    let xs : Vec<Point> = grid.all_points().into_iter().filter(|p|grid.get(p) == Some('X')).collect();
    let all_paths : Vec<Vec<Point>> = xs.iter().flat_map(|p|all_points_from(p)).collect();
    let strs = strings_from_points(&grid, all_paths);
    let count = strs.iter().filter(|s| *s == "MAS").count();
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input).unwrap().1;
    let result = grid
        .all_points()
        .iter()
        .filter(|p| grid.get(p) == Some('A'))
        .filter(|p| {
            let top_left = grid.get(&p.move_direction(&NW));
            let top_right = grid.get(&p.move_direction(&NE));
            let bottom_left = grid.get(&p.move_direction(&SW));
            let bottom_right = grid.get(&p.move_direction(&SE));

            let left_cross = (top_left == Some('M') && bottom_right == Some('S'))
                || (top_left == Some('S') && bottom_right == Some('M'));
            let right_cross = (bottom_left == Some('M') && top_right == Some('S'))
                || (bottom_left == Some('S') && top_right == Some('M'));

            return left_cross && right_cross;
        })
        .count();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pt1() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, data) = parse_input(&input).unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
