use advent_of_code::grid::Direction::{E, N, S, W};
use advent_of_code::grid::{Grid, Point};
use advent_of_code::search::dijkstra;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::{complete, map};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(18);

fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(
        tag("\n"),
        map(
            separated_pair(complete::i32, tag(","), complete::i32),
            |(x, y)| Point::new(x, y),
        ),
    )(input)
}

fn create_grid(walls: Vec<Point>) -> Grid<char> {
    let data: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];
    let mut grid = Grid::new(data);
    walls.iter().for_each(|p| grid.set(p, '#'));
    grid
}

fn neighbors(grid: &Grid<char>, point: &Point) -> Vec<(Point, u64)> {
    [N, S, E, W]
        .into_iter()
        .map(|d| point.move_direction(&d))
        .filter(|p| grid.get(p) == Some('.'))
        .map(|p| (p, 1))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let walls = parse_input(input)
        .unwrap()
        .1
        .into_iter()
        .take(1024)
        .collect();
    let grid = create_grid(walls);

    let start = Point::new(0, 0);
    let end = Point::new(70, 70);

    let is_goal = |point: &Point| *point == end;
    let nebs = |point: &Point| neighbors(&grid, point);

    let (_, cost) = dijkstra(&start, &nebs, &is_goal).unwrap();
    Some(cost)
}

pub fn part_two(input: &str) -> Option<String> {
    let start = Point::new(0, 0);
    let end = Point::new(70, 70);

    let is_goal = |point: &Point| *point == end;
    let all_walls = parse_input(input).unwrap().1;
    for i in 0..=all_walls.len() {
        let walls = all_walls.iter()
            .take(i)
            .cloned()
            .collect();
        let grid = create_grid(walls);
        let nebs = |point: &Point| neighbors(&grid, point);
        if dijkstra(&start, nebs, &is_goal).is_none() {
            if i > 0 {
                let blocking_point = all_walls.get(i - 1).unwrap();
                return Some(format!("{},{}", blocking_point.row, blocking_point.col));
            } else {
                return None;
            }
        }
    }

    None
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
