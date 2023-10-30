//! # Boiling Boulders
//!
//! The lava droplet is a fixed size so we can use a one dimensional fixed size array to store the
//! cube data for speed.
//!
//! For part two we use the [flood fill](https://en.wikipedia.org/wiki/Flood_fill) algorithm
//! starting from any corner to fill the outside space with water. We then use the same exposed
//! edge counting approach as part one, but only considering faces that touch a water drop.
use crate::util::iter::*;
use crate::util::parse::*;

const SIZE: usize = 22;

pub fn parse(input: &str) -> Vec<u32> {
    let mut cube = vec![0; SIZE * SIZE * SIZE];
    // Leave a 1 layer boundary around the outside for the part two flood fill
    // and also so that we don't have to use boundary checks when checking neighbors.
    input.iter_unsigned().chunk::<3>().for_each(|[x, y, z]: [usize; 3]| {
        cube[(x + 1) * SIZE * SIZE + (y + 1) * SIZE + (z + 1)] = 1;
    });
    cube
}

pub fn part1(input: &[u32]) -> u32 {
    // The exposed surface area is the 6 faces of the cubes minus any neighbors.
    count(input, |x| 6 - x)
}

pub fn part2(input: &[u32]) -> u32 {
    let mut cube = input.to_vec();
    // "Paint" the outside of the cube with water drops.
    flood_fill(&mut cube, 0);
    // Divide by 8 so that we only count water cubes.
    count(&cube, |x| x >> 3)
}

fn count(cube: &[u32], adjust: fn(u32) -> u32) -> u32 {
    let mut total = 0;

    for i in 0..cube.len() {
        if cube[i] == 1 {
            // No need for boundary checks as all cubes are at least 1 away from the edge.
            total += adjust(
                cube[i - 1]
                    + cube[i + 1]
                    + cube[i - SIZE]
                    + cube[i + SIZE]
                    + cube[i - SIZE * SIZE]
                    + cube[i + SIZE * SIZE],
            );
        }
    }

    total
}

fn flood_fill(cube: &mut [u32], i: usize) {
    if cube.get(i) == Some(&0) {
        // Use 8 as the nearest power of two greater than 6.
        cube[i] = 8;
        // We may wrap around to an opposite edge but that will also be water.
        flood_fill(cube, i.saturating_sub(1));
        flood_fill(cube, i + 1);
        flood_fill(cube, i.saturating_sub(SIZE));
        flood_fill(cube, i + SIZE);
        flood_fill(cube, i.saturating_sub(SIZE * SIZE));
        flood_fill(cube, i + SIZE * SIZE);
    }
}
