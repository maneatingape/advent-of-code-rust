use crate::util::point::*;
use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Grid<u8> {
        let raw: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid {
            width,
            height,
            bytes,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn default_copy<U: Default + Copy>(&self) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![U::default(); (self.width * self.height) as usize],
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point { x, y }
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.bytes[(self.width * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * point.y + point.x) as usize]
    }
}
