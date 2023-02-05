use crate::util::point::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    None,
    Elf,
    Proposal,
    Conflict,
}

pub struct Input {
    stride: usize,
    grid: Vec<Tile>,
    elves: Vec<Point>,
}

pub fn parse(_input: &str) -> Input {
    let raw: Vec<&[u8]> = _input
        .lines()
        .map(|line| line.as_bytes())
        .collect();
    let width = raw[0].len();
    let height = raw.len();
    let stride = 3 * width;
    let offset = Point {
        x: width as i32,
        y: height as i32,
    };

    let mut grid = vec![Tile::None; 9 * width * height];
    let mut elves = Vec::new();

    for (y, row) in raw.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                b'#' => {
                    grid[(y + height) * stride + (x + width)] = Tile::Elf;
                    elves.push(offset + Point { x: x as i32, y: y as i32 });
                },
                _ => (),
            }
        }
    }

    Input { stride, grid, elves }
}

pub fn part1(input: &Input) -> i32 {
    let len = input.elves.len();
    let stride = input.stride;
    let mut grid = input.grid.clone();
    let mut elves = input.elves.clone();
    let mut order = [UP, DOWN, LEFT, RIGHT];
    let mut proposals: Vec<(usize, Point)> = Vec::with_capacity(elves.len());

    for _ in 0..10 {
        step(len, stride, &mut grid, &mut elves, &mut order, &mut proposals);
    }

    let xs: Vec<i32> = elves.iter().map(|e| e.x).collect();
    let ys: Vec<i32> = elves.iter().map(|e| e.y).collect();
    let min_x = xs.iter().min().unwrap();
    let max_x = xs.iter().max().unwrap();
    let min_y = ys.iter().min().unwrap();
    let max_y = ys.iter().max().unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - (len as i32)
}

pub fn part2(input: &Input) -> i32 {
    let len = input.elves.len();
    let stride = input.stride;
    let mut grid = input.grid.clone();
    let mut elves = input.elves.clone();
    let mut order = [UP, DOWN, LEFT, RIGHT];
    let mut proposals: Vec<(usize, Point)> = Vec::with_capacity(elves.len());

    let mut moved = true;
    let mut count = 0;

    while moved {
        moved = step(len, stride, &mut grid, &mut elves, &mut order, &mut proposals);
        count += 1
    }

    count
}

fn step(len: usize, stride: usize, grid: &mut Vec<Tile>, elves: &mut Vec<Point>, order: &mut [Point; 4], proposals: &mut Vec<(usize, Point)>) -> bool {
    for i in 0..len {
        let elf = elves[i];
        let index = (elf.y as usize) * stride + (elf.x as usize);

        let nw = grid[index - stride - 1] == Tile::Elf;
        let n = grid[index - stride] == Tile::Elf;
        let ne = grid[index - stride + 1] == Tile::Elf;
        let w = grid[index - 1] == Tile::Elf;
        let e = grid[index + 1] == Tile::Elf;
        let sw = grid[index + stride - 1] == Tile::Elf;
        let s = grid[index + stride] == Tile::Elf;
        let se = grid[index + stride + 1] == Tile::Elf;

        if nw | n | ne | w | e | sw | s | se {
            for delta in order.iter() {
                let proceed = match *delta {
                    UP => !(n | ne | nw),
                    DOWN => !(s | se | sw),
                    LEFT => !(w | nw | sw),
                    RIGHT => !(e | ne | se),
                    _ => unreachable!(),
                };
                if proceed {
                    let next = elf + *delta;
                    let index = (next.y as usize) * stride + (next.x as usize);
                    let result = match grid[index] {
                        Tile::None => Tile::Proposal,
                        Tile::Proposal => Tile::Conflict,
                        _ => unreachable!(),
                    };
                    grid[index] = result;
                    if result != Tile::Conflict {
                        proposals.push((i, next));
                    }
                    break;
                }
            }
        }
    }

    let mut moved = false;

    for (i, next) in proposals.iter() {
        let index = (next.y as usize) * stride + (next.x as usize);
        let result = match grid[index] {
            Tile::Proposal => Tile::Elf,
            Tile::Conflict => Tile::None,
            _ => unreachable!(),
        };
        grid[index] = result;
        if result == Tile::Elf {
            let foo = elves[*i];
            let index = (foo.y as usize) * stride + (foo.x as usize);
            grid[index] = Tile::None;
            elves[*i] = *next;
            moved = true;
        }
    }

    proposals.clear();
    order.rotate_left(1);
    moved
}
