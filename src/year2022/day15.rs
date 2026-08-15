//! # Beacon Exclusion Zone
use std::ops::Range;

use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

pub struct Input {
    sensor: Point,
    beacon: Point,
    manhattan: i32,
}

pub fn parse(input: &str) -> Vec<Input> {
    input
        .iter_signed()
        .chunk::<4>()
        .map(|[x1, y1, x2, y2]| {
            let sensor = Point::new(x1, y1);
            let beacon = Point::new(x2, y2);
            Input { sensor, beacon, manhattan: sensor.manhattan(beacon) }
        })
        .collect()
}

/// The example uses y=10 but the real data uses y=2000000, so break out the logic
/// into a separate function to enable integration testing.
pub fn part1(input: &[Input]) -> i32 {
    part1_testable(input, 2_000_000)
}

/// A beacon cannot be located within the radius of a sensor unless it is the closest beacon.
///
/// We first convert each scanner's diamond shaped area into a one-dimensional range at the
/// specified row. By sorting the ranges, we can quickly calculate the total number of distinct
/// ranges where another beacon cannot exist, only counting overlapping areas once.
///
/// Beacons can also not be located at the same position as another beacon so we then also discount
/// any beacon located exactly on the specified row.
pub fn part1_testable(input: &[Input], row: i32) -> i32 {
    // Converts the "diamond" shaped area of each scanner into a one-dimensional row.
    // If the scanner's range does not reach the specified row then return `None`.
    fn build_range(input: &Input, row: i32) -> Option<Range<i32>> {
        let Input { sensor, manhattan, .. } = input;
        let extra = manhattan - (sensor.y - row).abs();
        (extra >= 0).then(|| (sensor.x - extra)..(sensor.x + extra))
    }

    // Sort the ranges first.
    let mut ranges: Vec<_> = input.iter().filter_map(|i| build_range(i, row)).collect();
    ranges.sort_unstable_by_key(|r| r.start);

    let mut total = 0;
    let mut max = i32::MIN;

    // Compare each range to the next.
    for Range { start, end } in ranges {
        if start > max {
            // If there is no overlap with the previous range, then add the entire length.
            total += end - start + 1;
            max = end;
        } else {
            // If some part of the range overlaps, then only add any extra length.
            // (it's possible that there is no extra length)
            total += (end - max).max(0);
            max = max.max(end);
        }
    }

    // Returns the x position of all beacons that are located on the specified row.
    let beacons: FastSet<_> =
        input.iter().filter_map(|i| (i.beacon.y == row).then_some(i.beacon.x)).collect();
    total - (beacons.len() as i32)
}

/// Similar to part one, the logic is broken out into a separate function to enable testing.
pub fn part2(input: &[Input]) -> u64 {
    part2_testable(input, 4_000_000)
}

/// The trick to solving this efficiently is to first *rotate* the corners of the diamond
/// scanner shape by 45 degrees. This transforms them into squares that make it much easier
/// to find the missing distress beacon.
///
/// Of the entire 4000000 by 4000000 area the missing beacon must be located in the only
/// square area not covered by a scanner.
pub fn part2_testable(input: &[Input], size: i32) -> u64 {
    let capacity = input.len();
    let mut top = FastSet::with_capacity(capacity);
    let mut left = FastSet::with_capacity(capacity);
    let mut bottom = FastSet::with_capacity(capacity);
    let mut right = FastSet::with_capacity(capacity);

    // Rotate points clockwise by 45 degrees, scale by √2 and extend edge by 1.
    // This transforms each sensor into an axis aligned bounding box.
    // The distress beacon is located where the top, left, bottom and right
    // edges of 4 separate bounding boxes intersect.
    for Input { sensor, manhattan, .. } in input {
        top.insert(sensor.x + sensor.y - manhattan - 1);
        left.insert(sensor.x - sensor.y - manhattan - 1);
        bottom.insert(sensor.x + sensor.y + manhattan + 1);
        right.insert(sensor.x - sensor.y + manhattan + 1);
    }

    let horizontal: Vec<_> = top.intersection(&bottom).copied().collect();
    let vertical: Vec<_> = left.intersection(&right).copied().collect();
    let range = 0..=size;

    // Many input files have vertical.len() == 1 and horizontal.len() == 1, which implies exactly
    // one answer; but this check covers more situations and is not too expensive.
    for &x in &vertical {
        for &y in &horizontal {
            // Rotate intersection point counter-clockwise and scale by 1 / √2
            // to return to original coordinates.
            #[expect(clippy::manual_midpoint)]
            let point = Point::new((x + y) / 2, (y - x) / 2);
            // As we're mixing overlaps from different boxes there may be some spurious false
            // positives, so double check all points are within the specified area
            // and outside the range of all scanners.
            if range.contains(&point.x)
                && range.contains(&point.y)
                && input.iter().all(|i| i.sensor.manhattan(point) > i.manhattan)
            {
                return 4_000_000 * (point.x as u64) + (point.y as u64);
            }
        }
    }

    unreachable!()
}
