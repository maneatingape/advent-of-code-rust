use crate::util::point::*;

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<u8>,
}

impl Grid {
    pub fn parse(input: &str) -> Grid {
        let raw: Vec<_> = input
            .lines()
            .map(|line| line.as_bytes())
            .collect();
        let width = raw[0].len();
        let height = raw.len();
        let mut bytes = Vec::with_capacity(width * height);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid { width, height, bytes }
    }

    pub fn empty_copy(&self) -> Grid {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![0; self.width * self.height],
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0
        && point.x < self.width as i32
        && point.y >= 0
        && point.y < self.height as i32
    }

    pub fn get(&self, point: Point) -> u8 {
        self.bytes[self.width * (point.y as usize) + (point.x as usize)]
    }

    pub fn set(&mut self, point: Point, value: u8) {
        self.bytes[self.width * (point.y as usize) + (point.x as usize)] = value;
    }

    pub fn find(&self, needle: u8) -> Option<Point> {
        let to_point = |index| {
            let x = (index % self.width) as i32;
            let y = (index / self.width) as i32;
            Point { x, y }
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }
}