use crate::util::collection::*;
use crate::util::parse::to_vec;
use crate::util::point::*;

pub struct State {
    cave: Vec<bool>,
    width: i32,
    height: i32,
    start: i32,
    floor: bool,
    count: i32,
}

impl State {
    fn fall(&mut self, unit: Point) -> bool {
        if self.cave[(unit.1 * self.width + unit.0) as usize] {
            true
        }
        else if unit.1 == self.height - 1 {
            self.floor
        }
        else if self.fall(Point(unit.0, unit.1 + 1))
            && self.fall(Point(unit.0 - 1, unit.1 + 1))
            && self.fall(Point(unit.0 + 1, unit.1 + 1))
        {
            self.cave[(unit.1 * self.width + unit.0) as usize] = true;
            self.count += 1;
            true
        }
        else {
            false
        }
    }
}

pub fn parse(input: &str) -> State {
    let points: Vec<Vec<i32>> = input.lines().map(to_vec::<i32>).collect();
    let max_y = points.iter().flat_map(|row| row.iter().skip(1).step_by(2).max()).max().unwrap();
    let width = 2 * max_y + 5;
    let height = max_y + 3;
    let start = max_y + 2;
    let mut cave = Vec::tabulate((width * height) as usize, |_| false);

    for row in points {
        for window in row.windows(4).step_by(2) {
            if let [x1, y1, x2, y2] = window {
                for x in *x1.min(x2)..=*x1.max(x2) {
                    for y in *y1.min(y2)..=*y1.max(y2) {
                        cave[((y * width) + (x - 500 + start)) as usize] = true;
                    }
                }
            }
        }
    }

    State { cave, width, height, start, floor: false, count: 0 }
}

pub fn part1(input: &State) -> i32 {
    simulate(input, false)
}

pub fn part2(input: &State) -> i32 {
    simulate(input, true)
}

fn simulate(input: &State, floor: bool) -> i32 {
    let mut state = State { cave: input.cave.clone(), floor, ..*input };
    state.fall(Point(state.start, 0));
    state.count
}
