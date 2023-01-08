use crate::util::collection::*;
use crate::util::parse::to_vec;
use crate::util::point::*;

type Input = (Vec<u8>, i32, i32, i32);

pub fn parse(input: &str) -> Input {
    let points: Vec<Vec<i32>> = input.lines().map(to_vec::<i32>).collect();
    let max_y = points.iter().flat_map(|row| row.iter().skip(1).step_by(2).max()).max().unwrap();

    let width = 2 * max_y + 5;
    let height = max_y + 3;
    let start = max_y + 2;
    let mut rock = Vec::tabulate((width * height) as usize, |_| 0u8);

    for row in points {
        for window in row.windows(4).step_by(2) {
            if let [x1, y1, x2, y2] = window {
                for x in *x1.min(x2)..=*x1.max(x2) {
                    for y in *y1.min(y2)..=*y1.max(y2) {
                        rock[((y * width) + (x - 500 + start)) as usize] = 1;
                    }
                }
            }
        }
    }

    (rock, width, height, start)
}

pub fn part1(input: &Input) -> usize {
    simulate(input, false)
}

pub fn part2(input: &Input) -> usize {
    simulate(input, true)
}

pub fn simulate(input: &Input, floor: bool) -> usize {
    fn helper(sand: &mut Vec<u8>, width: i32, height: i32, floor: bool, unit: Point) -> bool {
        if sand[(unit.1 * width + unit.0) as usize] != 0 {
            true
        }
        else if unit.1 == height - 1 {
            floor
        }
        else if helper(sand, width, height, floor, Point(unit.0, unit.1 + 1))
            && helper(sand, width, height, floor, Point(unit.0 - 1, unit.1 + 1))
            && helper(sand, width, height, floor, Point(unit.0 + 1, unit.1 + 1))
        {
            sand[(unit.1 * width + unit.0) as usize] = 2;
            true
        }
        else {
            false
        }
    }

    let (rock, width, height, start) = input;
    let mut sand = rock.clone();
    helper(&mut sand, *width, *height, floor, Point(*start, 0));
    sand.iter().filter(|&&s| s == 2).count()
}
