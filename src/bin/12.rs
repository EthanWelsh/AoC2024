use advent_of_code::grid::Direction::{E, N, S, W};
use advent_of_code::grid::{Direction, Grid, Point};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{line_ending, none_of};
use nom::combinator::{eof, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::iter::successors;

advent_of_code::solution!(12);

struct UnionFind {
    parent: HashMap<Point, Point>,
    rank: HashMap<Point, usize>,
}

impl UnionFind {
    fn new(points: &Vec<Point>) -> Self {
        UnionFind {
            parent: points.iter().map(|&p| (p, p)).collect(),
            rank: points.iter().map(|&p| (p, 0)).collect(),
        }
    }

    fn find(&self, p: &Point) -> Point {
        let parent = self.parent.get(p).unwrap();
        if parent == p {
            *p
        } else {
            self.find(parent)
        }
    }

    fn union(&mut self, a: &Point, b: &Point) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        let rank_a = *self.rank.get(&root_a).unwrap();
        let rank_b = *self.rank.get(&root_b).unwrap();

        if rank_a < rank_b {
            self.parent.insert(root_a, root_b);
            self.rank.entry(root_b).and_modify(|r| *r += rank_a);
        } else {
            self.parent.insert(root_b, root_a);
            self.rank.entry(root_a).and_modify(|r| *r += rank_b);
        }
    }

    fn connected_components(&self) -> Vec<HashSet<Point>> {
        let mut root_map: HashMap<Point, HashSet<Point>> = HashMap::new();
        self.parent.keys().for_each(|p| {
            let root = self.find(p);
            root_map.entry(root).or_insert_with(HashSet::new).insert(*p);
        });

        root_map.values().cloned().collect()
    }
}

fn parse_input(input: &str) -> IResult<&str, Grid<char>> {
    let parse_line = terminated(many1(none_of(" \t\n\r")), alt((line_ending, eof)));
    map(many1(parse_line), |data| Grid::new(data))(input)
}

fn neighbors(grid: &Grid<char>, point: Point) -> Vec<Point> {
    let current_letter = grid.get(&point).unwrap();
    [N, S, E, W]
        .map(|d| point.move_direction(&d))
        .into_iter()
        .filter(|p| grid.get(p) == Some(current_letter))
        .collect_vec()
}

fn get_perimeter(grid: &Grid<char>, points: &HashSet<Point>) -> u64 {
    let current_letter = grid.get(points.iter().next().unwrap()).unwrap();
    points
        .iter()
        .cartesian_product([N, S, E, W])
        .map(|(p, d)| p.move_direction(&d))
        .filter(|p| grid.get(p) != Some(current_letter))
        .count() as u64
}

fn get_plot_points(grid: &Grid<char>) -> Vec<HashSet<Point>> {
    let points = grid.all_points();
    let mut union_find = UnionFind::new(&points);

    for p in points.iter() {
        for n in neighbors(grid, *p) {
            union_find.union(p, &n);
        }
    }

    union_find.connected_components()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input).unwrap().1;
    let plot_points = get_plot_points(&grid);
    let ret = plot_points
        .into_iter()
        .map(|ps| {
            let area = ps.len() as u64;
            let perimeter = get_perimeter(&grid, &ps);
            return area * perimeter;
        })
        .sum();

    Some(ret)
}

fn expand(
    point: &Point,
    direction: &Direction,
    outer_points_and_dir: &HashSet<(Point, Direction)>,
) -> Vec<Point> {
    let outer_points: HashSet<Point> = outer_points_and_dir
        .iter()
        .filter(|(p, d)| d == direction)
        .map(|(p, _)| *p)
        .collect();

    let expand_dirs = match *direction {
        N | S => [E, W],
        E | W => [N, S],
        _ => panic!("invalid direction"),
    };

    expand_dirs
        .into_iter()
        .flat_map(|dir| {
            let outer_points = outer_points.clone();
            successors(Some(*point), move |p| {
                let next = p.move_direction(&dir);
                outer_points.contains(&next).then_some(next)
            })
        })
        .sorted()
        .dedup()
        .collect()
}

fn get_sides(grid: &Grid<char>, points: &HashSet<Point>) -> u64 {
    let current_letter = grid.get(points.iter().next().unwrap()).unwrap();
    let outer_points_and_dir: HashSet<(Point, Direction)> = points
        .iter()
        .cartesian_product([N, S, E, W])
        .map(|(p, d)| (p.move_direction(&d), d))
        .filter(|(p, d)| grid.get(p) != Some(current_letter))
        .collect();

    let mut sides: HashMap<Direction, HashSet<Vec<Point>>> = HashMap::new();
    for (p, d) in outer_points_and_dir.iter() {
        let line = expand(p, d, &outer_points_and_dir);
        sides.entry(*d).or_insert_with(HashSet::new).insert(line);
    }
    sides.values().map(|lines| lines.len()).sum::<usize>() as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input).unwrap().1;
    let plot_points = get_plot_points(&grid);
    let ret = plot_points
        .into_iter()
        .map(|ps| {
            let area = ps.len() as u64;
            let sides = get_sides(&grid, &ps);
            return area * sides;
        })
        .sum();

    Some(ret)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_1() {
        let puzzle = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        let result = part_two(puzzle);
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_2() {
        let puzzle = indoc! {"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        "};
        let result = part_two(puzzle);
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_3() {
        let puzzle = indoc! {"
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
        "};
        let result = part_two(puzzle);
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_4() {
        let puzzle = indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "};
        let result = part_two(puzzle);
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_5() {
        let puzzle = indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};
        let result = part_two(puzzle);
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_6() {
        let puzzle = indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};
        let result = part_two(puzzle);
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_7() {
        let puzzle = indoc! {"
            AAAA
            AABA
            ABBA
            AABA
        "};
        let result = part_two(puzzle);
        assert_eq!(result, Some(176));
    }
}
