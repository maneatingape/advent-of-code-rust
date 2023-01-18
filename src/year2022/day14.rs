use crate::util::collection::*;
use crate::util::parse::*;

pub struct Cave {
    sand: Vec<bool>,
    width: u32,
    height: u32,
    start: u32,
    floor: bool,
    count: u32,
}

impl Cave {
    fn fall(&mut self, x: u32, y: u32) -> bool {
        let index = (y * self.width + x) as usize;
        if self.sand[index] {
            true
        } else if y == self.height - 1 {
            self.floor
        } else if self.fall(x, y + 1) && self.fall(x - 1, y + 1) && self.fall(x + 1, y + 1) {
            self.sand[index] = true;
            self.count += 1;
            true
        } else {
            false
        }
    }
}

pub fn parse(input: &str) -> Cave {
    let points: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.to_unsigned_iter().collect())
        .collect();
    let max_y = points
        .iter()
        .flat_map(|row| row.iter().skip(1).step_by(2).max())
        .max()
        .unwrap();
    let width = 2 * max_y + 5;
    let height = max_y + 3;
    let start = max_y + 2;
    let mut sand = Vec::fill((width * height) as usize, false);

    for row in points {
        for window in row.windows(4).step_by(2) {
            if let [x1, y1, x2, y2] = window {
                for x in *x1.min(x2)..=*x1.max(x2) {
                    for y in *y1.min(y2)..=*y1.max(y2) {
                        sand[((y * width) + (x + start - 500)) as usize] = true;
                    }
                }
            }
        }
    }

    Cave { sand, width, height, start, floor: false, count: 0 }
}

pub fn part1(input: &Cave) -> u32 {
    simulate(input, false)
}

pub fn part2(input: &Cave) -> u32 {
    simulate(input, true)
}

fn simulate(input: &Cave, floor: bool) -> u32 {
    let mut cave = Cave {
        sand: input.sand.clone(),
        floor,
        ..*input
    };
    cave.fall(cave.start, 0);
    cave.count
}
