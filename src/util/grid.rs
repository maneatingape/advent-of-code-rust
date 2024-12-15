//! Fast 2 dimensional Grid backed by a single `vec`. This module is designed to work with [`Point`].
//!
//! The traits [`Index`] and [`IndexMut`] are implemented for [`Point`] to allow usage like:
//!
//! ```
//!   # use aoc::util::grid::Grid;
//!   # use aoc::util::point::Point;
//!
//!   let mut grid = Grid::parse("1");
//!   let point = Point::new(0, 0);
//!
//!   let foo = grid[point];
//!   assert_eq!(foo, b'1');
//!
//!   grid[point] = foo + 1;
//!   assert_eq!(grid[point], b'2');
//! ```
//!
//! A convenience [`parse`] method creates a `Grid` directly from a 2 dimenionsal set of
//! ASCII characters, a common occurence in Advent of Code inputs. The [`same_size_with`] function
//! creates a grid of the same size, that can be used for in BFS algorithms for tracking visited
//! location or for tracking cost in Djikstra.
//!
//! [`Point`]: crate::util::point
//! [`parse`]: Grid::parse
//! [`same_size_with`]: Grid::same_size_with
use crate::util::point::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    #[inline]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid { width, height, bytes }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);
                print!("{}", self[point] as char);
            }
            println!();
        }
        println!();
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid { width, height, bytes: vec![value; (width * height) as usize] }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![value; (self.width * self.height) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}
