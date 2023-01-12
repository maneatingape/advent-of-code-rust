use crate::util::parse::*;
use crate::util::point::*;
use std::ops::Range;
use std::collections::HashSet;

pub struct Input {
    sensor: Point,
    beacon: Point,
    manhattan: i32,
}

pub fn parse(input: &str) -> Vec<Input> {
    fn helper(line: &str) -> Input {
        let tokens = to_signed_vec::<i32>(line);
        let sensor = Point(tokens[0], tokens[1]);
        let beacon = Point(tokens[2], tokens[3]);
        let manhattan = sensor.manhattan(beacon);
        Input { sensor, beacon, manhattan }
    }
    input.lines().map(helper).collect()
}

pub fn part1(input: &[Input]) -> i32 {
    part1_testable(input, 2_000_000)
}

pub fn part1_testable(input: &[Input], row: i32) -> i32 {
    fn build_range(input: &Input, row: i32) -> Option<Range<i32>> {
        let Input { sensor, beacon: _, manhattan } = input;
        let extra = manhattan - (sensor.1 - row).abs();
        if extra >= 0 { Some((sensor.0 - extra)..(sensor.0 + extra)) } else { None }
    }

    let mut ranges: Vec<Range<i32>> = input.iter().filter_map(|i| build_range(i, row)).collect();
    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    let mut total = 0;
    let mut max = i32::MIN;

    for Range { start, end } in ranges {
        if start > max {
            total += end - start + 1;
            max = end;
        } else {
            total += (end - max).max(0);
            max = max.max(end)
        }
    }

    fn build_beacons(input: &Input, row:  i32) -> Option<i32> {
        let Input { sensor: _, beacon, manhattan: _ } = input;
        if beacon.1 == row { Some(beacon.0) } else { None }
    }
    let beacons: HashSet<i32> = input.iter().filter_map(|i| build_beacons(i, row)).collect();

    total - (beacons.len() as i32)
}

pub fn part2(input: &[Input]) -> u64 {
    part2_testable(input, 4_000_000)
}

pub fn part2_testable(input: &[Input], size: i32) -> u64 {
    let mut top: HashSet<i32> = HashSet::new();
    let mut left: HashSet<i32> = HashSet::new();
    let mut bottom: HashSet<i32> = HashSet::new();
    let mut right: HashSet<i32> = HashSet::new();

    // Rotate points clockwise by 45 degrees, scale by √2 and extend edge by 1.
    // This transform each sensor into an axis aligned bounding box.
    // The distress beacon is located where the top, left, bottom and right
    // edges of 4 separate bounding boxes intersect.
    for Input { sensor, beacon: _, manhattan } in input.iter() {
        top.insert(sensor.0 + sensor.1 - manhattan - 1);
        left.insert(sensor.0 - sensor.1 - manhattan - 1);
        bottom.insert(sensor.0 + sensor.1 + manhattan + 1);
        right.insert(sensor.0 - sensor.1 + manhattan + 1);
    }

    let horizontal: Vec<&i32> = top.intersection(&bottom).collect();
    let vertical: Vec<&i32> = left.intersection(&right).collect();
    let range = 0..(size + 1);

    for x in vertical.iter() {
        for y in horizontal.iter() {
            // Rotate intersection point counter clockwise and scale by 1 / √2
            // to return to original coordinates.
            let point = Point((**x + **y) / 2, (**y - **x) / 2);
            if range.contains(&point.0) && range.contains(&point.1)
                && input.iter().all(|i| i.sensor.manhattan(point) > i.manhattan) {
                return 4_000_000 * (point.0 as u64) + (point.1 as u64);
            }
        }
    }

    unreachable!()
}
