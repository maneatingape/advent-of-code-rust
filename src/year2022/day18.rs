use crate::util::iter::*;
use crate::util::parse::*;

const SIZE: usize = 22;

pub fn parse(input: &str) -> Vec<u32> {
    let mut cube = vec![0; SIZE * SIZE * SIZE];
    input
        .iter_unsigned()
        .chunk::<3>()
        .for_each(|[x, y, z]: [usize; 3]| {
            cube[(x + 1) * SIZE * SIZE + (y + 1) * SIZE + (z + 1)] = 1;
        });
    cube
}

pub fn part1(input: &[u32]) -> u32 {
    count(input, |x| 6 - x)
}

pub fn part2(input: &[u32]) -> u32 {
    let mut cube = input.to_vec();
    flood_fill(&mut cube, 0);
    count(&cube, |x| x >> 3)
}

fn count(cube: &[u32], adjust: fn(u32) -> u32) -> u32 {
    let mut total = 0;

    for i in 0..cube.len() {
        if cube[i] == 1 {
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
        cube[i] = 8;
        flood_fill(cube, i.saturating_sub(1));
        flood_fill(cube, i + 1);
        flood_fill(cube, i.saturating_sub(SIZE));
        flood_fill(cube, i + SIZE);
        flood_fill(cube, i.saturating_sub(SIZE * SIZE));
        flood_fill(cube, i + SIZE * SIZE);
    }
}
