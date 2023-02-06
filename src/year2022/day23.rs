const NORTH: u32 = 0b11100000;
const SOUTH: u32 = 0b00000111;
const WEST: u32 =  0b10010100;
const EAST: u32 =  0b00101001;
const ORDER: u32 = NORTH + (SOUTH << 8) + (WEST << 16) + (EAST << 24);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    None,
    Proposal,
    Conflict,
}

#[derive(Clone, Copy)]
struct Todo {
    i: usize,
    next: usize,
    direction: u32,
}

#[derive(Clone)]
pub struct Input {
    stride: usize,
    grid: Vec<u8>,
    elves: Vec<usize>,
    proposals: Vec<Tile>,
    todo: Vec<Todo>,
}

pub fn parse(_input: &str) -> Input {
    let raw: Vec<&[u8]> = _input
        .lines()
        .map(|line| line.as_bytes())
        .collect();
    let width = raw[0].len();
    let height = raw.len();

    let stride = 3 * width;
    let mut grid = vec![0; 9 * width * height];
    let mut elves = Vec::new();
    let proposals = vec![Tile::None; elves.len()];
    let todo = Vec::with_capacity(elves.len());

    for (y, row) in raw.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == b'#' {
                let index = (y + height) * stride + (x + width);
                elves.push(index);

                grid[index - stride - 1] |= 0b00000001;
                grid[index - stride]     |= 0b00000010;
                grid[index - stride + 1] |= 0b00000100;
                grid[index - 1]          |= 0b00001000;
                grid[index + 1]          |= 0b00010000;
                grid[index + stride - 1] |= 0b00100000;
                grid[index + stride]     |= 0b01000000;
                grid[index + stride + 1] |= 0b10000000;
            }
        }
    }

    Input { stride, grid, elves, proposals, todo }
}

pub fn part1(input: &Input) -> usize {
    let mut state = input.clone();
    let mut order = ORDER;

    state.proposals = vec![Tile::None; state.grid.len()];

    for _ in 0..10 {
        step(&mut state, order);
        order = order.rotate_right(8);
    }

    let Input { elves, stride, .. } = state;
    let xs: Vec<_> = elves.iter().map(|e| e % stride).collect();
    let ys: Vec<_> = elves.iter().map(|e| e / stride).collect();
    let min_x = xs.iter().min().unwrap();
    let max_x = xs.iter().max().unwrap();
    let min_y = ys.iter().min().unwrap();
    let max_y = ys.iter().max().unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len()
}

pub fn part2(input: &Input) -> i32 {
    let mut state = input.clone();
    let mut order = ORDER;
    let mut moved = true;
    let mut count = 0;

    state.proposals = vec![Tile::None; state.grid.len()];

    while moved {
        moved = step(&mut state, order);
        order = order.rotate_right(8);
        count += 1;
    }

    count
}

fn step(state: &mut Input, order: u32) -> bool {
    let Input { stride, grid, elves, proposals, todo, .. } = state;
    let mut moved = false;

    for (i, &elf) in elves.iter().enumerate() {
        let neighbors = grid[elf] as u32;

        if neighbors != 0 {
            let first = order & 0xff;
            if first & neighbors == 0 {
                propose(i, elf, first, *stride, proposals, todo);
                continue;
            }

            let second = (order >> 8) & 0xff;
            if second & neighbors == 0 {
                propose(i, elf, second, *stride, proposals, todo);
                continue;
            }

            let third = (order >> 16) & 0xff;
            if third & neighbors == 0 {
                propose(i, elf, third, *stride, proposals, todo);
                continue;
            }

            let fourth = (order >> 24) & 0xff;
            if fourth & neighbors == 0 {
                propose(i, elf, fourth, *stride, proposals, todo);
                continue;
            }
        }
    }

    for &Todo { i, next, direction } in todo.iter() {
        if proposals[next] == Tile::Proposal {
            elves[i] = next;
            moved = true;
            update(next, direction, *stride, grid);
        }
        proposals[next] = Tile::None;
    }

    todo.clear();
    moved
}

#[inline]
fn propose(i: usize, elf: usize, direction: u32, stride: usize, proposals: &mut Vec<Tile>, todo: &mut Vec<Todo>) {
    let next = match direction {
        NORTH => elf - stride,
        SOUTH => elf + stride,
        WEST => elf - 1,
        EAST => elf + 1,
        _ => unreachable!(),
    };

    if proposals[next] == Tile::None {
        proposals[next] = Tile::Proposal;
        todo.push(Todo { i, next, direction });
    } else {
        proposals[next] = Tile::Conflict;
    }
}

#[inline]
fn update(next: usize, direction: u32, stride: usize, grid: &mut Vec<u8>) {
    match direction {
        NORTH => {
            grid[next - stride - 1]     ^= 0b00000001;
            grid[next - stride]         ^= 0b00000010;
            grid[next - stride + 1]     ^= 0b00000100;
            grid[next - 1]              ^= 0b00001001;
            grid[next]                  ^= 0b00000010;
            grid[next + 1]              ^= 0b00010100;
            grid[next + stride - 1]     ^= 0b00101000;
            grid[next + stride]         ^= 0b01000000;
            grid[next + stride + 1]     ^= 0b10010000;
            grid[next + 2 * stride - 1] ^= 0b00100000;
            grid[next + 2 * stride]     ^= 0b01000000;
            grid[next + 2 * stride + 1] ^= 0b10000000;
        },
        SOUTH => {
            grid[next + stride - 1]     ^= 0b00100000;
            grid[next + stride]         ^= 0b01000000;
            grid[next + stride + 1]     ^= 0b10000000;
            grid[next - 1]              ^= 0b00101000;
            grid[next]                  ^= 0b01000000;
            grid[next + 1]              ^= 0b10010000;
            grid[next - stride - 1]     ^= 0b00001001;
            grid[next - stride]         ^= 0b00000010;
            grid[next - stride + 1]     ^= 0b00010100;
            grid[next - 2 * stride - 1] ^= 0b00000001;
            grid[next - 2 * stride]     ^= 0b00000010;
            grid[next - 2 * stride + 1] ^= 0b00000100;
        },
        WEST => {
            grid[next - stride - 1] ^= 0b00000001;
            grid[next - stride]     ^= 0b00000011;
            grid[next - stride + 1] ^= 0b00000110;
            grid[next - stride + 2] ^= 0b00000100;
            grid[next - 1]          ^= 0b00001000;
            grid[next]              ^= 0b00001000;
            grid[next + 1]          ^= 0b00010000;
            grid[next + 2]          ^= 0b00010000;
            grid[next + stride - 1] ^= 0b00100000;
            grid[next + stride]     ^= 0b01100000;
            grid[next + stride + 1] ^= 0b11000000;
            grid[next + stride + 2] ^= 0b10000000;
        },
        EAST => {
            grid[next - stride + 1] ^= 0b00000100;
            grid[next - stride]     ^= 0b00000110;
            grid[next - stride - 1] ^= 0b00000011;
            grid[next - stride - 2] ^= 0b00000001;
            grid[next + 1]          ^= 0b00010000;
            grid[next]              ^= 0b00010000;
            grid[next - 1]          ^= 0b00001000;
            grid[next - 2]          ^= 0b00001000;
            grid[next + stride + 1] ^= 0b10000000;
            grid[next + stride]     ^= 0b11000000;
            grid[next + stride - 1] ^= 0b01100000;
            grid[next + stride - 2] ^= 0b00100000;
        },
        _ => unreachable!(),
    }
}
