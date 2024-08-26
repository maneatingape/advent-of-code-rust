//! # A Regular Map
//!
//! Simple solution taking advantage of a controversial property of the input. After taking any
//! branch it's assumed that we can return to the pre-branch position. This does *not* hold for
//! general inputs, as it's easy to construct paths which violate this constraint.
//!
//! We use a stack to save the position before a branch, pushing whenever an opening `(` is
//! encountered then popping whenever the closing `)` is found. Additionally we assume that
//! the location will never move more than 55 rooms from the starting location in order to use
//! a fixed size array to hold the minimum distance to any room.
type Input = (u32, usize);

pub fn parse(input: &str) -> Input {
    explore(input)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

fn explore(input: &str) -> Input {
    // Start in the center.
    let mut index = 6105;
    // 55 in each direction, gives a width and height of 110, for a total size of 12,100.
    let mut grid = vec![u32::MAX; 12_100];
    let mut stack = Vec::with_capacity(500);
    let mut part_one = 0;

    grid[index] = 0;

    for b in input.bytes() {
        let distance = grid[index];

        match b {
            b'(' => stack.push(index),
            b'|' => index = stack[stack.len() - 1],
            b')' => index = stack.pop().unwrap(),
            b'N' => index -= 110,
            b'S' => index += 110,
            b'W' => index -= 1,
            b'E' => index += 1,
            _ => (),
        }

        grid[index] = grid[index].min(distance + 1);
        part_one = part_one.max(grid[index]);
    }

    let part_two = grid.iter().filter(|d| (1000..u32::MAX).contains(d)).count();
    (part_one, part_two)
}
