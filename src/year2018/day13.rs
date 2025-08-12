//! # Mine Cart Madness
//!
//! Simulates the mine carts for both parts. When checking the grid we only care about `\`, `/`
//! and `+` characters, all other characters can be ignored. Carts are sorted by `y` and then by
//! `x` before each tick as the movement order is important to resolve collisions or near misses
//! correctly.
use crate::util::grid::*;
use crate::util::point::*;

pub struct Input {
    grid: Grid<u8>,
    carts: Vec<Cart>,
}

#[derive(Clone, Copy)]
pub struct Cart {
    position: Point,
    direction: Point,
    turns: u8,
    active: bool,
}

impl Cart {
    fn new(position: Point, direction: Point) -> Cart {
        Cart { position, direction, turns: 0, active: true }
    }

    fn tick(&mut self, grid: &Grid<u8>) {
        self.position += self.direction;

        match grid[self.position] {
            b'\\' => self.direction = Point::new(self.direction.y, self.direction.x),
            b'/' => self.direction = Point::new(-self.direction.y, -self.direction.x),
            b'+' => {
                self.direction = match self.turns {
                    0 => self.direction.counter_clockwise(),
                    1 => self.direction,
                    _ => self.direction.clockwise(), // 2 turns
                };
                self.turns = (self.turns + 1) % 3;
            }
            _ => (),
        }
    }
}

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut carts = Vec::new();

    for (i, b) in grid.bytes.iter().enumerate() {
        let direction = match b {
            b'^' => UP,
            b'v' => DOWN,
            b'<' => LEFT,
            b'>' => RIGHT,
            _ => continue,
        };

        let x = i as i32 % grid.width;
        let y = i as i32 / grid.width;
        carts.push(Cart::new(Point::new(x, y), direction));
    }

    Input { grid, carts }
}

pub fn part1(input: &Input) -> String {
    let mut carts = input.carts.clone();
    let mut occupied = input.grid.same_size_with(false);

    loop {
        // Turn order is important.
        carts.sort_unstable_by_key(|c| input.grid.width * c.position.y + c.position.x);

        for cart in &mut carts {
            // Follow tracks to next position.
            occupied[cart.position] = false;
            cart.tick(&input.grid);
            let next = cart.position;

            if occupied[next] {
                return format!("{},{}", next.x, next.y);
            }

            occupied[next] = true;
        }
    }
}

pub fn part2(input: &Input) -> String {
    let mut carts = input.carts.clone();
    let mut occupied = input.grid.same_size_with(false);

    while carts.len() > 1 {
        // Turn order is important.
        carts.sort_unstable_by_key(|c| input.grid.width * c.position.y + c.position.x);

        for i in 0..carts.len() {
            // Crashed carts may not have been removed yet.
            if carts[i].active {
                // Follow tracks to next position.
                occupied[carts[i].position] = false;
                carts[i].tick(&input.grid);
                let next = carts[i].position;

                if occupied[next] {
                    // Mark both carts as crashed.
                    carts.iter_mut().filter(|c| c.position == next).for_each(|c| c.active = false);
                    occupied[next] = false;
                } else {
                    occupied[next] = true;
                }
            }
        }

        // Removed crashed carts to speed up future ticks.
        carts.retain(|c| c.active);
    }

    let last = carts[0].position;
    format!("{},{}", last.x, last.y)
}
