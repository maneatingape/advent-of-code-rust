use crate::util::parse::*;
use Kind::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Air,
    Falling,
    Stopped,
}

#[derive(Clone)]
pub struct Cave {
    width: usize,
    height: usize,
    size: usize,
    kind: Vec<Kind>,
    floor: Kind,
    count: u32,
}

impl Cave {
    fn fall(&mut self, index: usize) -> Kind {
        let result = self.check(index + self.width)
            && self.check(index + self.width - 1)
            && self.check(index + self.width + 1);

        if result {
            self.count += 1;
            self.kind[index] = Stopped;
            Stopped
        } else {
            self.kind[index] = Falling;
            Falling
        }
    }

    fn check(&mut self, index: usize) -> bool {
        let kind = if index >= self.size {
            self.floor
        } else if self.kind[index] == Air {
            self.fall(index)
        } else {
            self.kind[index]
        };
        kind == Stopped
    }
}

pub fn parse(input: &str) -> Cave {
    let unsigned = |line: &str| line.iter_unsigned().collect();
    let points: Vec<Vec<usize>> = input.lines().map(unsigned).collect();
    let max_y = points.iter().flat_map(|row| row.iter().skip(1).step_by(2)).max().unwrap();
    let width = 2 * max_y + 5;
    let height = max_y + 2;
    let size = width * height;
    let mut kind = vec![Air; size];

    for row in points {
        for window in row.windows(4).step_by(2) {
            if let &[x1, y1, x2, y2] = window {
                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        kind[(width * y) + (x + height - 500)] = Stopped;
                    }
                }
            }
        }
    }

    Cave { width, height, size, kind, floor: Air, count: 0 }
}

pub fn part1(input: &Cave) -> u32 {
    simulate(input, Falling)
}

pub fn part2(input: &Cave) -> u32 {
    simulate(input, Stopped)
}

fn simulate(input: &Cave, floor: Kind) -> u32 {
    let mut cave = input.clone();
    cave.floor = floor;
    cave.fall(cave.height);
    cave.count
}
