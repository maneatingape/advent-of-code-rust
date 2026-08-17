//! Fast 2-dimensional Grid backed by a single `vec`, designed to work with [`Point`].
//!
//! The traits [`Index`] and [`IndexMut`] are implemented for [`Point`] to allow usage like:
//!
//! ```
//! # use aoc::util::grid::Grid;
//! # use aoc::util::point::Point;
//!
//! let mut grid = Grid::parse("1");
//! let point = Point::new(0, 0);
//!
//! let foo = grid[point];
//! assert_eq!(foo, b'1');
//!
//! grid[point] = foo + 1;
//! assert_eq!(grid[point], b'2');
//! ```
//!
//! Two convenience methods, [`parse`] and [`parse_with_border`], create a `Grid` directly from a
//! 2-dimensional set of ASCII characters, a common occurrence in Advent of Code inputs. The former
//! strips all newlines, and [`contains`] is then useful to prevent accidental wraparound between
//! lines. The latter not only preserves newlines in the input, but adds a row of newlines above and
//! below, for algorithms where newline serves as a natural barrier without needing to use
//! [`contains`]. The [`same_size_with`] function creates a grid of the same size that can be used
//! in BFS algorithms for tracking visited locations or for tracking cost in Dijkstra.
//!
//! [`Point`]: crate::util::point
//! [`parse`]: Grid::parse
//! [`parse_with_border`]: Grid::parse_with_border
//! [`contains`]: Grid::contains
//! [`same_size_with`]: Grid::same_size_with
use std::ops::{Index, IndexMut};

use crate::util::point::*;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();

        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let bytes = raw.concat();

        Self { width, height, bytes }
    }

    #[must_use]
    pub fn parse_with_border(input: &str) -> Self {
        // Size things large enough so that both orthogonal and diagonal access hits a newline.
        // This shifts 0,0 to 1,1. Non-newline iteration would be `1..height-1` and `1..width`,
        // although it still often faster to iterate `0..height` and `0..width` when visiting
        // newline is harmless. For convenience, the allocation is oversized to compensate for unit
        // tests that omit a trailing newline.
        let width = input.lines().next().unwrap().len() + 1;
        let height = input.len().div_ceil(width) + 2;
        let size = width * height + 1;
        let mut bytes = Vec::with_capacity(size);

        bytes.resize(width + 1, b'\n');
        bytes.extend_from_slice(input.as_bytes());
        bytes.resize(size, b'\n');

        Self { width: width as i32, height: height as i32, bytes }
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
    #[must_use]
    pub fn find(&self, needle: T) -> Option<Point> {
        self.bytes
            .iter()
            .position(|&h| h == needle)
            .map(|index| Point::new(index as i32 % self.width, index as i32 / self.width))
    }
}

impl<T: Copy> Grid<T> {
    #[must_use]
    pub fn new(width: i32, height: i32, value: T) -> Self {
        Self { width, height, bytes: vec![value; (width * height) as usize] }
    }

    #[must_use]
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid::new(self.width, self.height, value)
    }
}

impl<T> Grid<T> {
    #[inline]
    #[must_use]
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
