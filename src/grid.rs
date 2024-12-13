use std::collections::HashMap;
use itertools::Itertools;
use num_traits::ToPrimitive;
use crate::grid::Direction::{N, E, S, W, NE, SE, NW, SW};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Direction {
    N, E, S, W, NE, SE, NW, SW
}

impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn subtract(&self, point: &Self) -> Point {
        Point::new(self.row - point.row, self.col - point.col)
    }

    pub fn add(&self, point: &Point) -> Point {
        Point::new(self.row + point.row, self.col + point.col)
    }

    pub fn move_direction(&self, dir: &Direction) -> Point {
        match dir {
            N => Point::new(self.row - 1, self.col),
            S => Point::new(self.row + 1, self.col),
            E => Point::new(self.row, self.col + 1),
            W => Point::new(self.row, self.col - 1),
            NW => Point::new(self.row - 1, self.col - 1),
            SW => Point::new(self.row + 1, self.col - 1),
            NE => Point::new(self.row - 1, self.col + 1),
            SE => Point::new(self.row + 1, self.col + 1),
        }
    }

    pub fn move_directions<'a, I>(&self, directions: I) -> Vec<Point>
    where
        I: IntoIterator<Item = &'a Direction>,
    {
        directions
            .into_iter()
            .scan(*self, |p, d| {
                *p = p.move_direction(d);
                Some(*p)
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: HashMap<Point, T>,
}

impl<T> Grid<T>
where
    T: Default + Clone + PartialEq + Copy,
{
    pub fn new_empty(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            data: HashMap::new()
        }
    }

    pub fn new(input: Vec<Vec<T>>) -> Self {
        let height = input.len();
        let width = input[0].len();

        let mut data = HashMap::with_capacity(width * height);
        for r in 0..height {
            for c in 0..width {
                data.insert(Point::new(r as i32, c as i32), input[r][c].clone());
            }
        }

        Grid {
            width,
            height,
            data
        }
    }

    pub fn get(&self, point: &Point) -> Option<T> {
        self.data.get(point).cloned()
    }

    pub fn set(&mut self, point: &Point, position: T) {
        if let Some(d) = self.data.get_mut(point) {
            *d = position;
        }
    }

    pub fn in_bounds(&self, point: &Point) -> bool {
        let Point { row, col } = point;
        *row >= 0 && *row < self.height.to_i32().unwrap() && *col >= 0 && *col < self.width.to_i32().unwrap()
    }

    pub fn all_points(&self) -> Vec<Point> {
        (0..self.height as i32)
            .cartesian_product(0..self.width as i32)
            .map(|(r, c)| Point { row: r, col: c })
            .collect()
    }

    pub fn neighbors(&self, point: &Point) -> Vec<Point> {
        let dirs = vec![N, E, S, W, NE, NW, SE, SW];
        dirs.iter().map(|d| point.move_direction(d))
            .filter(|p| self.in_bounds(p))
            .collect()
    }

    pub fn transform(&self, f: &dyn Fn(&Grid<T>, &Point) -> T) -> Grid<T> {
        let mut result = self.clone();
        self.all_points().iter().for_each(|point| {
            let position = f(&self, &point);
            result.set(&point, position);
        });
        result
    }
}
