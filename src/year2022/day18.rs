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

const SIZE: usize = 24;

pub fn parse(input: &str) -> Vec<u8> {
    let mut cube = vec![0; SIZE * SIZE * SIZE];
    // Leave a 1 layer boundary around the outside for the part two flood fill
    // and also so that we don't have to use boundary checks when checking neighbors.
    input.iter_unsigned().chunk::<3>().for_each(|[x, y, z]: [usize; 3]| {
        cube[(x + 1) * SIZE * SIZE + (y + 1) * SIZE + (z + 1)] = 1;
    });
    cube
}

pub fn part1(input: &[u8]) -> u32 {
    // The exposed surface area is the 6 faces of the cubes minus any neighbors.
    count(input, |x| 6 - x)
}

pub fn part2(input: &[u8]) -> u32 {
    // "Paint" the outside of the cube with water drops.
    // Use 8 as the nearest power of two greater than 6.
    let mut cube = input.to_vec();
    cube[0] = 8;

    let mut todo = Vec::new();
    todo.push(0);

    while let Some(index) = todo.pop() {
        let mut flood_fill = |next| {
            if next < input.len() && cube[next] == 0 {
                cube[next] = 8;
                todo.push(next);
            }
        };

        // We may wrap around but that index will be out of bounds.
        flood_fill(index.wrapping_sub(1));
        flood_fill(index + 1);
        flood_fill(index.wrapping_sub(SIZE));
        flood_fill(index + SIZE);
        flood_fill(index.wrapping_sub(SIZE * SIZE));
        flood_fill(index + SIZE * SIZE);
    }

    // Divide by 8 so that we only count water cubes.
    count(&cube, |x| x >> 3)
}

fn count(cube: &[u8], adjust: fn(u32) -> u32) -> u32 {
    let mut total = 0;

    for (i, &cell) in cube.iter().enumerate() {
        if cell == 1 {
            // No need for boundary checks as all cubes are at least 1 away from the edge.
            let neighbors = cube[i - 1] as u32
                + cube[i + 1] as u32
                + cube[i - SIZE] as u32
                + cube[i + SIZE] as u32
                + cube[i - SIZE * SIZE] as u32
                + cube[i + SIZE * SIZE] as u32;
            total += adjust(neighbors);
        }
    }

    total
}
