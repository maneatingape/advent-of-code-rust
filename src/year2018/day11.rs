//! # Chronal Charge
//!
//! Building a [summed-area table](https://en.wikipedia.org/wiki/Summed-area_table) allows us
//! to compute the power of any rectangle with only 4 array lookups.
//!
//! This makes the total complexity `O(n³)`, however the calculation for each size is independent
//! so we can parallelize over multiple threads.
use crate::util::parse::*;
use crate::util::thread::*;
use std::sync::Mutex;

pub struct Result {
    x: usize,
    y: usize,
    size: usize,
    power: i32,
}

struct Shared {
    sat: Vec<i32>,
    mutex: Mutex<Vec<Result>>,
}

pub fn parse(input: &str) -> Vec<Result> {
    let grid_serial_number: i32 = input.signed();

    // Build Summed-area table.
    let mut sat = vec![0; 301 * 301];

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
    let shared = Shared { sat, mutex: Mutex::new(Vec::new()) };
    spawn_parallel_iterator(&items, |iter| worker(&shared, iter));
    shared.mutex.into_inner().unwrap()
}

pub fn part1(input: &[Result]) -> String {
    let Result { x, y, .. } = input.iter().find(|r| r.size == 3).unwrap();
    format!("{x},{y}")
}

pub fn part2(input: &[Result]) -> String {
    let Result { x, y, size, .. } = input.iter().max_by_key(|r| r.power).unwrap();
    format!("{x},{y},{size}")
}

fn worker(shared: &Shared, iter: ParIter<'_, usize>) {
    let result: Vec<_> = iter
        .map(|&size| {
            let (power, x, y) = square(&shared.sat, size);
            Result { x, y, size, power }
        })
        .collect();

    shared.mutex.lock().unwrap().extend(result);
}

/// Find the (x,y) coordinates and max power for a square of the specified size.
fn square(sat: &[i32], size: usize) -> (i32, usize, usize) {
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

    (max_power, max_x, max_y)
}
