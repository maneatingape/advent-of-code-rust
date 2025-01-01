//! # Beverage Bandits
//!
//! This problem is notoriously tricky due to the finicky rules that must be followed precisely and
//! that not all inputs trigger all edge cases. However from a performance aspect most of the time
//! is consumed finding the nearest target whenever a unit needs to move.
//!
//! For each move we perform two [BFS](https://en.wikipedia.org/wiki/Breadth-first_search).
//! The first search from the current unit finds the nearest target in reading order.
//! The second *reverse* search from the target to the current unit finds the correct direction
//! to move.
//!
//! Since the cave dimensions are 32 x 32 we use a fixed sized array of bitmasks stored in `u32`
//! to execute each BFS efficiently. Each step we expand the frontier using the bitwise logic
//! applied to each row:
//!
//!  ```none
//!     (previous | (current << 1) | current | (current >> 1) | next) & !walls
//! ```
//!
//! We represent the goal using bits and stop searching once that intersects with the frontier.
//! First example:
//!
//! * Goblin's turn.
//! * We should choose the first target square in reading order (to the right of the nearest elf)
//! * There are two equal shortest paths to that square, so we should choose the first *step* in
//!   reading order (up).
//!
//! ```none
//!     Map        Walls      In Range
//!     #######    1111111    0000000
//!     #E    #    1000001    0110000
//!     # E   #    1000001    0111000
//!     #    G#    1000001    0010000
//!     #######    1111111    0000000
//!
//!     Forward BFS frontier                        Intersection
//!     0000000    0000000    0000000    0000000    0000000
//!     0000000    0000000    0000010    0000110    0000000
//!     0000000 => 0000010 => 0000110 => 0001110 => 0001000 <= Choose first target square
//!     0000010    0000110    0001110    0011110    0010000    in reading order
//!     0000000    0000000    0000000    0000000    0000000
//!
//!     Reverse BFS frontier             Intersection
//!     0000000    0000000    0000000    0000000
//!     0000000    0001000    0011100    0000000
//!     0001000 => 0011100 => 0111110 => 0000010 <= Choose first step
//!     0000000    0001000    0011100    0000100    in reading order
//!     0000000    0000000    0000000    0000000
//! ```
//!
//! Choosing the first intersection in reading order the Goblin correctly moves upwards.
//! Second example:
//!
//! * Elf's turn.
//! * There are two equal shortest paths.
//! * We should choose the first *unit* in reading order (left).
//!
//! ```none
//!     Map             Walls           In Range
//!     ###########    11111111111    00000000000
//!     #G..#....G#    10001000001    01100000110
//!     ###..E#####    11100011111    00000000000
//!     ###########    11111111111    00000000000
//!
//!     Forward BFS frontier                                                       Intersection
//!     00000000000    00000000000    00000000000    00000000000    00000000000    00000000000
//!     00000000000    00000100000    00000110000    00010111000    00110111100    00100000100
//!     00000100000 => 00001100000 => 00011100000 => 00011100000 => 00011100000 => 00000000000
//!     00000000000    00000000000    00000000000    00000000000    00000000000    00000000000
//!
//!     Reverse BFS frontier                                        Intersection
//!     00000000000    00000000000    00000000000    00000000000    00000000000
//!     00100000000    01110000000    01110000000    01110000000    00000000000
//!     00000000000 => 00000000000 => 00010000000 => 00011000000 => 00001000000
//!     00000000000    00000000000    00000000000    00000000000    00000000000
//! ```
//!
//! Choosing the first intersection in reading order the Elf correctly moves left.
use crate::util::grid::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::mpsc::{Sender, channel};

const READING_ORDER: [Point; 4] = [UP, LEFT, RIGHT, DOWN];

pub struct Input {
    walls: [u32; 32],
    elves: Vec<Point>,
    goblins: Vec<Point>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Elf,
    Goblin,
}

#[derive(Clone, Copy)]
struct Unit {
    position: Point,
    kind: Kind,
    health: i32,
    power: i32,
}

/// Shared between threads for part two.
struct Shared {
    done: AtomicBool,
    elf_attack_power: AtomicI32,
    tx: Sender<(i32, i32)>,
}

/// Parse the input into a bitmask for the cave walls
/// and a list of point coordinates for each Elf and Goblin.
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let mut walls = [0; 32];
    let mut elves = Vec::new();
    let mut goblins = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let position = Point::new(x, y);

            match grid[position] {
                b'#' => set_bit(&mut walls, position),
                b'E' => elves.push(position),
                b'G' => goblins.push(position),
                _ => (),
            }
        }
    }

    Input { walls, elves, goblins }
}

/// Simulate a full fight until only Goblins remain.
pub fn part1(input: &Input) -> i32 {
    fight(input, 3, false).unwrap()
}

/// Find the lowest attack power where no Elf dies. We can short circuit any fight once a
/// single Elf is killed. Since each fight is independent we can parallelize the search over
/// multiple threads.
pub fn part2(input: &Input) -> i32 {
    let (tx, rx) = channel();
    let shared = Shared { done: AtomicBool::new(false), elf_attack_power: AtomicI32::new(4), tx };

    // Use as many cores as possible to parallelize the search.
    spawn(|| worker(input, &shared));

    // Hang up the channel.
    drop(shared.tx);
    // Find lowest possible power.
    rx.iter().min_by_key(|&(eap, _)| eap).map(|(_, score)| score).unwrap()
}

fn worker(input: &Input, shared: &Shared) {
    while !shared.done.load(Ordering::Relaxed) {
        // Get the next attack power, incrementing it atomically for the next fight.
        let power = shared.elf_attack_power.fetch_add(1, Ordering::Relaxed);

        // If the Elves win then set the score and signal all threads to stop.
        // Use a channel to queue all potential scores as another thread may already have sent a
        // different value.
        if let Some(score) = fight(input, power, true) {
            shared.done.store(true, Ordering::Relaxed);
            let _unused = shared.tx.send((power, score));
        }
    }
}

/// Careful implementation of the game rules.
fn fight(input: &Input, elf_attack_power: i32, part_two: bool) -> Option<i32> {
    let mut units = Vec::new();
    let mut elves = input.elves.len();
    let mut goblins = input.goblins.len();
    let mut grid = Grid::new(32, 32, None);

    // Initialize each unit.
    for &position in &input.elves {
        units.push(Unit { position, kind: Kind::Elf, health: 200, power: elf_attack_power });
    }
    for &position in &input.goblins {
        units.push(Unit { position, kind: Kind::Goblin, health: 200, power: 3 });
    }

    for turn in 0.. {
        // Remove dead units for efficiency.
        units.retain(|u| u.health > 0);
        // Units take turns in reading order.
        units.sort_unstable_by_key(|u| 32 * u.position.y + u.position.x);
        // Grid is used for reverse lookup from location to index.
        units.iter().enumerate().for_each(|(i, u)| grid[u.position] = Some(i));

        for index in 0..units.len() {
            let Unit { position, kind, health, power } = units[index];

            // Unit may have been killed during this turn.
            if health <= 0 {
                continue;
            }

            // Check if there are no more remaining targets then return *complete* turns.
            // Determining a complete turn is subtle. If the last unit to act (in reading order)
            // kills the last remaining enemy then that counts as a complete turn. Otherwise the
            // turn is considered incomplete and doesn't count.
            if elves == 0 || goblins == 0 {
                return Some(turn * units.iter().map(|u| u.health.max(0)).sum::<i32>());
            }

            // Search for neighboring enemies.
            let mut nearby = attack(&grid, &units, position, kind);

            // If no enemy next to unit then move towards nearest enemy in reading order,
            // breaking equal distance ties in reading order.
            if nearby.is_none() {
                if let Some(next) = double_bfs(input.walls, &units, position, kind) {
                    grid[position] = None;
                    grid[next] = Some(index);
                    units[index].position = next;

                    nearby = attack(&grid, &units, next, kind);
                }
            }

            // Attack enemy if possible.
            if let Some(target) = nearby {
                units[target].health -= power;

                if units[target].health <= 0 {
                    grid[units[target].position] = None;

                    // For part two, short circuit if a single elf is killed.
                    match units[target].kind {
                        Kind::Elf if part_two => return None,
                        Kind::Elf => elves -= 1,
                        Kind::Goblin => goblins -= 1,
                    }
                }
            }
        }
    }

    unreachable!()
}

/// Search for weakest neighboring enemy. Equal health ties are broken in reading order.
fn attack(grid: &Grid<Option<usize>>, units: &[Unit], point: Point, kind: Kind) -> Option<usize> {
    let mut enemy_health = i32::MAX;
    let mut enemy_index = None;

    for next in READING_ORDER.iter().filter_map(|&o| grid[point + o]) {
        if units[next].kind != kind && units[next].health < enemy_health {
            enemy_health = units[next].health;
            enemy_index = Some(next);
        }
    }

    enemy_index
}

/// Performs two BFS searches. The first search from the current unit finds the nearest target
/// in reading order. The second reverse search from the target to the current unit, finds the
/// correct direction to move.
fn double_bfs(mut walls: [u32; 32], units: &[Unit], point: Point, kind: Kind) -> Option<Point> {
    let frontier = &mut [0; 32];
    set_bit(frontier, point);

    let walls = &mut walls;
    let in_range = &mut [0; 32];

    for unit in units.iter().filter(|u| u.health > 0) {
        if unit.kind == kind {
            // Units of the same type are obstacles.
            set_bit(walls, unit.position);
        } else {
            // Add enemy units to the list of potential targets.
            set_bit(in_range, unit.position);
        }
    }

    // We're interested in the 4 orthogonal squares around each enemy unit.
    expand(walls, in_range);

    // Search for reachable squares. There could be no reachable squares, for example friendly
    // units already have the enemy surrounded or are blocking the path.
    while expand(walls, frontier) {
        if let Some(target) = intersect(in_range, frontier) {
            // Reverse search from target to determine correct movement direction.
            let frontier = &mut [0; 32];
            set_bit(frontier, target);

            let in_range = &mut [0; 32];
            set_bit(in_range, point);
            expand(walls, in_range);

            // This will always succeed as there was a path from the current unit.
            loop {
                expand(walls, frontier);
                if let Some(target) = intersect(in_range, frontier) {
                    return Some(target);
                }
            }
        }
    }

    None
}

/// Use bitwise logic to expand the frontier. Returns a boolean indicating if the frontier
/// actually expanded.
fn expand(walls: &[u32], frontier: &mut [u32]) -> bool {
    let mut previous = frontier[0];
    let mut changed = 0;

    for i in 1..31 {
        let current = frontier[i];
        let next = frontier[i + 1];

        frontier[i] = (previous | (current << 1) | current | (current >> 1) | next) & !walls[i];

        previous = current;
        changed |= current ^ frontier[i];
    }

    changed != 0
}

/// Check if we have reached a target, returning the first target in reading order.
fn intersect(in_range: &[u32], frontier: &[u32]) -> Option<Point> {
    for i in 1..31 {
        let both = in_range[i] & frontier[i];

        if both != 0 {
            let x = both.trailing_zeros() as i32;
            let y = i as i32;
            return Some(Point::new(x, y));
        }
    }

    None
}

/// Convenience function to set a single bit from a point's location.
#[inline]
fn set_bit(slice: &mut [u32], point: Point) {
    slice[point.y as usize] |= 1 << point.x;
}
