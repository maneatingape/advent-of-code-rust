//! # Four-Dimensional Adventure
//!
//! This problem is the classic [union find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure).
//! However since we only need the *count* of the distinct sets we can use a much simpler approach.
//!
//! Starting with an arbitrary point we find all other points within range, adding them to a
//! todo list. We then transitively determine the neighbors of those points, and so on until
//! all sets have been found.
//!
//! The simplest way to check neighbors is to do a quadratic search: compute a distance for
//! every point compared against every other remaining point.  But for our input files ranging
//! from 1000 to 1300 points, (n²+n)/2 requires well over half a million distance computations.
//! Inspecting the input file, there are no digits outside [-8,8] in any of the four dimensions;
//! and even when you consider points looking at neighbors up to distance three away, that still
//! means we are never probing any dimension outside the range [-11,11].  And 23^4 is merely
//! 279841 points, which is easy enough to express in memory as a flat 1D array or via a
//! `FastSet`.  What's more, there are exactly 128 points in a 3x3x3x3 hypersphere surrounding
//! any given point; this is small enough to translate into a lookup table of 128 offsets to
//! a 1D location to see if another point resides at that offset.  With that in hand, we
//! can now complete the nearest neighbor search with 128*n existence checks, which easily beats
//! (n²+n)/2 Manhattan distance computations for inputs with more than 1000 points.
use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    // Collapse inputs into a single positive base-23 number offset from -11,-11,-11,-11
    input
        .iter_signed::<i32>()
        .chunk::<4>()
        .map(|[x, y, z, w]| flatten(x, y, z, w, 11) as usize)
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    let mut constellations = 0;
    let mut todo = Vec::with_capacity(input.len());

    // Populate a hashtable of all points that still need a constellation.
    let mut remaining = [false; 23_usize.pow(4)];
    input.iter().for_each(|&point| remaining[point] = true);

    // Build up the list of interesting offsets.
    let mut offsets = Vec::with_capacity(128);
    for x in -3_i32..4 {
        let lim_y = 3 - x.abs();
        for y in -lim_y..lim_y + 1 {
            let lim_z = lim_y - y.abs();
            for z in -lim_z..lim_z + 1 {
                let lim_w = lim_z - z.abs();
                for w in -lim_w..lim_w + 1 {
                    if x != 0 || y != 0 || z != 0 || w != 0 {
                        offsets.push(flatten(x, y, z, w, 0) as isize);
                    }
                }
            }
        }
    }
    assert_eq!(offsets.len(), 128);

    // Choose arbitrary point and start a new constellation.
    for &point in input {
        if !remaining[point] {
            continue; // Already part of a constellation
        }
        constellations += 1;
        todo.push(point);
        remaining[point] = false;

        while let Some(index) = todo.pop() {
            // Find all neighbors, adding them to `todo` in order to transitively find all
            // other points in the constellation.
            for &offset in &offsets {
                let candidate = index.wrapping_add_signed(offset);
                if remaining[candidate] {
                    todo.push(candidate);
                    remaining[candidate] = false;
                }
            }
        }
    }

    constellations
}

pub fn part2(_input: &[usize]) -> &'static str {
    "n/a"
}

// Flatten a point into a 1D base-23 number relative to an offset.
fn flatten(x: i32, y: i32, z: i32, w: i32, offset: i32) -> i32 {
    (x + offset) * 23 * 23 * 23 + (y + offset) * 23 * 23 + (z + offset) * 23 + (w + offset)
}
