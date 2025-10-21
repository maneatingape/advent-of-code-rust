//! # Chronal Charge
//!
//! Building a [summed-area table](https://en.wikipedia.org/wiki/Summed-area_table) allows us
//! to compute the power of any rectangle with only 4 array lookups.
//!
//! This makes the total complexity `O(n³)`, however the calculation for each size is independent
//! so we can parallelize over multiple threads.
use crate::util::parse::*;
use crate::util::thread::*;

pub struct Result {
    size: usize,
    x: usize,
    y: usize,
    power: i32,
}

pub fn parse(input: &str) -> Vec<Result> {
    let grid_serial_number: i32 = input.signed();

    // Build Summed-area table. Add a little extra buffer to the end for the SIMD variant.
    let mut sat = vec![0; 301 * 301 + 32];

    for y in 1..301 {
        for x in 1..301 {
            let rack_id = x + 10;

            let mut power_level = rack_id * y;
            power_level += grid_serial_number;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            power_level -= 5;

            let index = (301 * y + x) as usize;
            sat[index] = power_level + sat[index - 1] + sat[index - 301] - sat[index - 302];
        }
    }

    // Use as many cores as possible to parallelize the search.
    // Smaller sizes take more time so use work stealing to keep all cores busy.
    let items: Vec<_> = (1..301).collect();
    let result = spawn_parallel_iterator(&items, |iter| {
        iter.map(|&size| square(&sat, size)).collect::<Vec<_>>()
    });
    result.into_iter().flatten().collect()
}

pub fn part1(input: &[Result]) -> String {
    let Result { x, y, .. } = input.iter().find(|r| r.size == 3).unwrap();
    format!("{x},{y}")
}

pub fn part2(input: &[Result]) -> String {
    let Result { size, x, y, .. } = input.iter().max_by_key(|r| r.power).unwrap();
    format!("{x},{y},{size}")
}

/// Find the (x,y) coordinates and max power for a square of the specified size.
#[cfg(not(feature = "simd"))]
fn square(sat: &[i32], size: usize) -> Result {
    let mut max_power = i32::MIN;
    let mut max_x = 0;
    let mut max_y = 0;

    for y in size..301 {
        for x in size..301 {
            let index = 301 * y + x;

            let power =
                sat[index] - sat[index - size] - sat[index - 301 * size] + sat[index - 302 * size];

            if power > max_power {
                max_power = power;
                max_x = x - size + 1;
                max_y = y - size + 1;
            }
        }
    }

    Result { size, x: max_x, y: max_y, power: max_power }
}

/// Same as the scalar version but processing 16 lanes simultaneously.
#[cfg(feature = "simd")]
fn square(sat: &[i32], size: usize) -> Result {
    use std::simd::cmp::SimdPartialOrd as _;
    use std::simd::*;

    const LANE_WIDTH: usize = 16;
    type Vector = Simd<i32, LANE_WIDTH>;

    let mut max_power = i32::MIN;
    let mut max_x = 0;
    let mut max_y = 0;

    for y in size..301 {
        for x in (size..301).step_by(LANE_WIDTH) {
            let index = 301 * y + x;

            let power: Vector = Simd::from_slice(&sat[index..])
                - Simd::from_slice(&sat[index - size..])
                - Simd::from_slice(&sat[index - 301 * size..])
                + Simd::from_slice(&sat[index - 302 * size..]);

            if power.simd_gt(Simd::splat(max_power)).any() {
                let limit = 301 - x;
                for (offset, power) in power.to_array().into_iter().enumerate().take(limit) {
                    if power > max_power {
                        max_power = power;
                        max_x = x - size + 1 + offset;
                        max_y = y - size + 1;
                    }
                }
            }
        }
    }

    Result { size, x: max_x, y: max_y, power: max_power }
}
