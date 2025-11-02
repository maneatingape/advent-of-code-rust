//! # Amphipod
//!
//! Our high-level approach is an [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) search
//! over all possible burrow states. Three techniques are used to speed things up.
//!
//! Firstly a good choice of heuristic is crucial. The heuristic used has the following
//! characteristics:
//! * Exactly correct for optimal moves.
//! * Cheap to update on each subsequent move.
//!
//! Secondly pruning states to reduce the search space is very beneficial. Two approaches are used:
//! * A cache of previously seen states. If amphipods are in the same position but with a higher
//!   cost then the current state will never be optimal and can be pruned.
//! * Detecting deadlocked states where an amphipod in the hallway prevents any possible solution.
//!   Exploring any further is a waste of time.
//!
//! Thirdly low-level bit manipulation is used to represent the burrow state size compactly
//! in only 16 bytes for faster copying and hashing.
use crate::util::hash::*;
use crate::util::heap::*;
use std::array::from_fn;
use std::hash::*;

/// The values of `A`, `B`, `C` and `D` are used heavily to calculate room indices.
const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const ROOM: usize = 4;
const EMPTY: usize = 5;
const COST: [usize; 4] = [1, 10, 100, 1000];

/// Pack the room state into only 2 bytes.
///
/// We use 3 bits for each amphipod plus a marker bit for a maximum of 13 bits. The room is a
/// stack with the amphipod closest to the hallway in the least significant position.
///
/// The marker bit is used to determine how full a room is and to disambiguate empty from the `A`
/// type.
///
/// Some example rooms:
/// * Empty room `0000000000000001`
/// * Room with two `A`s `0000000001000000`
/// * Room with `ABCD` where `A` is closest to hallway `0001011010001000`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Room {
    packed: u16,
}

impl Room {
    /// Pack state into a compact `u16` representation.
    fn new(spaces: [usize; 4]) -> Room {
        let packed = (1 << 12) | (spaces[0] << 9) | (spaces[1] << 6) | (spaces[2] << 3) | spaces[3];
        Room { packed: packed as u16 }
    }

    /// The marker bit is always in the most significant position, so can be used to find out the
    /// size of a room.
    fn size(self) -> usize {
        ((15 - self.packed.leading_zeros()) / 3) as usize
    }

    /// Find the type of an amphipod closest to the hallway.
    fn peek(self) -> Option<usize> {
        (self.packed > 1).then_some((self.packed & 0b111) as usize)
    }

    /// Remove the top amphipod.
    fn pop(&mut self) -> usize {
        let pod = (self.packed & 0b111) as usize;
        self.packed >>= 3;
        pod
    }

    /// A room is "open" if amphipods of that type can move to it. This means that it must be
    /// empty or only already contain amphipods of that type.
    ///
    /// We use a multiplication by a constant to figure out the bit pattern. For example a room
    /// with three `B`s would have a bit pattern of `0000001001001001` which is the marker bit
    /// plus B << 6 + B << 3 + B << 0 = B * 64 + B * 8 + B = B * 73.
    fn open(self, kind: usize) -> bool {
        self.packed == 1
            || self.packed == (1 << 3) + (kind as u16) // 1
            || self.packed == (1 << 6) + (kind as u16 * 9) // 8 + 1
            || self.packed == (1 << 9) + (kind as u16 * 73) // 64 + 8 + 1
            || self.packed == (1 << 12) + (kind as u16 * 585) // 512 + 64 + 8 + 1
    }

    /// Return an amphipod to the correct room.
    fn push(&mut self, kind: usize) {
        self.packed = (self.packed << 3) | (kind as u16);
    }

    /// Returns the amphipod at a specific index from the *bottom* of the burrow.
    /// 0 is the bottom amphipod furthest from the hallway, 1 the next closest and so on.
    fn spaces(self, index: usize) -> usize {
        let adjusted = 3 * (self.size() - 1 - index);
        ((self.packed >> adjusted) & 0b111) as usize
    }
}

/// Pack the state of the hallway into a `usize`. Each hallway position is represented by a nibble
/// with the pod type (plus additionally empty or room entrance markers) for a total of 44 bits.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Hallway {
    packed: usize,
}

impl Hallway {
    /// The initial hallway is empty. Room entrances are marked as type 4.
    fn new() -> Hallway {
        Hallway { packed: 0x55454545455 }
    }

    /// Find the amphipod at a specific location.
    fn get(self, index: usize) -> usize {
        (self.packed >> (index * 4)) & 0xf
    }

    /// Updated the amphipod at a specific location.
    fn set(&mut self, index: usize, value: usize) {
        let mask = !(0xf << (index * 4));
        let value = value << (index * 4);
        self.packed = (self.packed & mask) | value;
    }
}

/// Combine hallway and four rooms into a complete burrow representation in only
/// 8 + 4 * 2 = 16 bytes.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Burrow {
    hallway: Hallway,
    rooms: [Room; 4],
}

impl Burrow {
    fn new(rooms: [[usize; 4]; 4]) -> Burrow {
        Burrow { hallway: Hallway::new(), rooms: from_fn(|i| Room::new(rooms[i])) }
    }
}

/// Subtracts the ASCII value of `A` from each character of the input so that amphipod values
/// match the constants defined above.
pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b.saturating_sub(b'A') as usize).collect())
        .collect()
}

/// Part one is a special case of the full burrow where two amphipods of each type are already
/// in the correct position in each room.
pub fn part1(input: &[Vec<usize>]) -> usize {
    let burrow = Burrow::new([
        [A, A, input[3][3], input[2][3]],
        [B, B, input[3][5], input[2][5]],
        [C, C, input[3][7], input[2][7]],
        [D, D, input[3][9], input[2][9]],
    ]);
    organize(burrow)
}

/// Part two adds the middle amphipods as specified in the problem statement.
pub fn part2(input: &[Vec<usize>]) -> usize {
    let burrow = Burrow::new([
        [input[3][3], D, D, input[2][3]],
        [input[3][5], B, C, input[2][5]],
        [input[3][7], A, B, input[2][7]],
        [input[3][9], C, A, input[2][9]],
    ]);
    organize(burrow)
}

/// A* search over all possible burrow states until we find the lowest cost to organize.
///
/// Each state is processed in one of two phases, "condense" or "expand".
///
/// In condense, amphipods move from the hallway or another burrow directly to their home burrow.
/// Multiple moves are combined if possible and each burrow is tried from left to right.
/// In terms of energy this is always an optimal move.
///
/// If no moves to home burrows are possible then the expand phase moves amphipods into the
/// hallway.
fn organize(burrow: Burrow) -> usize {
    let mut todo = MinHeap::with_capacity(20_000);
    let mut seen = FastMap::with_capacity(20_000);

    // Initial calculation of the heuristic is expensive but future updates will be cheap.
    todo.push(best_possible(&burrow), burrow);

    while let Some((energy, mut burrow)) = todo.pop() {
        let open: [bool; 4] = from_fn(|i| burrow.rooms[i].open(i));

        // Process each burrow that is open in left to right order. More than one amphipod may move.
        let mut changed = false;
        for (i, &open) in open.iter().enumerate() {
            if open && burrow.rooms[i].size() < 4 {
                let offset = 2 + 2 * i;
                let forward = (offset + 1)..11;
                let reverse = (0..offset).rev();
                changed |= condense(&mut burrow, i, forward);
                changed |= condense(&mut burrow, i, reverse);
            }
        }

        if changed {
            // If amphipods moved back to their home burrow in the condense phase then
            // check if we're fully organized.
            if burrow.rooms.iter().enumerate().all(|(i, r)| open[i] && r.size() == 4) {
                return energy;
            }

            // Moving back to home burrow does not change total energy due to the way the
            // heuristic is calculated. For example if we have spent 100 energy and the heuristic
            // is 100, spending 10 to move an amphipod would result in 110 energy spent and a
            // heuristic of 90.
            let min = seen.get(&burrow).unwrap_or(&usize::MAX);
            if energy < *min {
                todo.push(energy, burrow);
                seen.insert(burrow, energy);
            }
        } else {
            // If no amphipods can return to their home burrow then fan out into multiple states
            // by moving the top amphipod from each burrow into the hallway.
            for (i, &open) in open.iter().enumerate() {
                if !open {
                    let offset = 2 + 2 * i;
                    let forward = (offset + 1)..11;
                    let reverse = (0..offset).rev();
                    expand(&mut todo, &mut seen, burrow, energy, i, forward);
                    expand(&mut todo, &mut seen, burrow, energy, i, reverse);
                }
            }
        }
    }

    unreachable!()
}

/// Heuristic of the lowest possible energy to organize the burrow. Assumes that amphipods can
/// move through the hallway unblocked.
fn best_possible(burrow: &Burrow) -> usize {
    let mut energy = 0;
    // How many of each kind are outside their home burrow. Used to adjust the energy needed
    // to move. The first amphipod will need to move all the way to the bottom, but the next
    // will only need to move 1 space less.
    let mut need_to_move = [0; 4];

    for (original_kind, room) in burrow.rooms.iter().enumerate() {
        let mut blocker = false;

        // Search from bottom to top
        for depth in 0..room.size() {
            let kind = room.spaces(depth);
            if kind != original_kind {
                // Any amphipod above us will need to move out of the way.
                blocker = true;
                need_to_move[kind] += 1;
                // Calculate the energy to return directly to our home burrow
                // taking into account how many other amphipods of our kind also need to move.
                let up = 4 - depth;
                let across = 2 * kind.abs_diff(original_kind); // Distance between rooms.
                let down = need_to_move[kind];
                energy += COST[kind] * (up + across + down);
            } else if blocker {
                // Even though we're in our home burrow we need to move out of the way of a lower
                // amphipod of a different kind.
                need_to_move[kind] += 1;
                // Calculate the energy assuming we can move to one of the nearest hallway spaces
                // on either side.
                let up = 4 - depth;
                let across = 2; // Nearest spot then back
                let down = need_to_move[kind];
                energy += COST[kind] * (up + across + down);
            }
        }
    }

    energy
}

/// Starting from a burrow of a specific kind, searches the hallway and other rooms from either
/// left or right direction, returning all amphipods of that kind to the burrow.
/// Stops searching immediately if blocked.
fn condense(burrow: &mut Burrow, kind: usize, iter: impl Iterator<Item = usize>) -> bool {
    let mut changed = false;

    for hallway_index in iter {
        match burrow.hallway.get(hallway_index) {
            // Skip over empty spaces.
            EMPTY => (),
            // Move as many amphipods as possible from the room to their home burrow.
            ROOM => {
                let room_index = (hallway_index - 2) / 2;

                while burrow.rooms[room_index].peek() == Some(kind) {
                    burrow.rooms[room_index].pop();
                    burrow.rooms[kind].push(kind);
                    changed = true;
                }
            }
            // Move from hallway to home burrow.
            pod if pod == kind => {
                burrow.hallway.set(hallway_index, EMPTY);
                burrow.rooms[kind].push(kind);
                changed = true;
            }
            // We're blocked from any further progress in this direction.
            _ => break,
        }
    }

    changed
}

/// Searches the hallway in either the right or left direction, pushing a new state to the
/// priority queue if it's possible to place an amphipod there.
fn expand(
    todo: &mut MinHeap<usize, Burrow>,
    seen: &mut FastMap<Burrow, usize>,
    mut burrow: Burrow,
    energy: usize,
    room_index: usize,
    iter: impl Iterator<Item = usize>,
) {
    let kind = burrow.rooms[room_index].pop();

    for hallway_index in iter {
        match burrow.hallway.get(hallway_index) {
            // Amphipods can't stop directly outside rooms.
            ROOM => (),
            // Check each empty space
            EMPTY => {
                let mut next = burrow;
                next.hallway.set(hallway_index, kind);

                // If this move would result in a state that can never be finished then prune early.
                if deadlock_left(&next)
                    || deadlock_right(&next)
                    || deadlock_room(&next, 0)
                    || deadlock_room(&next, 1)
                    || deadlock_room(&next, 2)
                    || deadlock_room(&next, 3)
                {
                    continue;
                }

                // If the destination is outside of the direct path from our current burrow
                // to our home burrow then add the extra energy to move there *and back* to the
                // heuristic.
                let start = 2 + 2 * room_index;
                let end = 2 + 2 * kind;

                let adjust = if start == end {
                    // If in our home burrow but moving out of the way of another kind,
                    // then assume the minimum possible distance of 1 place to either the
                    // left or right in the hallway.
                    let across = hallway_index.abs_diff(start);
                    across - 1
                } else {
                    let lower = start.min(end);
                    let upper = start.max(end);
                    // One of these expressions will be zero depending on direction.
                    lower.saturating_sub(hallway_index) + hallway_index.saturating_sub(upper)
                };

                let extra = COST[kind] * 2 * adjust;

                // Critical optimization. If we're not in our home burrow then we must move out of
                // the way otherwise we'd become a blocker.
                if kind != room_index && extra == 0 {
                    continue;
                }

                // Check that we haven't already seen this state before with lower energy
                // in order to prune suboptimal duplicates.
                let next_energy = energy + extra;
                let min = seen.get(&next).unwrap_or(&usize::MAX);

                if next_energy < *min {
                    todo.push(next_energy, next);
                    seen.insert(next, next_energy);
                }
            }
            // We're blocked from any further progress in this direction.
            _ => break,
        }
    }
}

/// Checks for a situation where an `A` amphipod can block other amphipods in the leftmost burrow.
///
/// For example:
/// ```none
///     #############
///     #...A.......#
///     ### #.#.#.###
///       #A#.#.#.#
///       #A#.#.#.#
///       #B#.#.#.#
///       #########
/// ```
///
/// The top two `A`s can move into the left hallways spaces but the `B` will then be stuck
/// and we'll never be able to organize the burrow completely.
fn deadlock_left(burrow: &Burrow) -> bool {
    let room = &burrow.rooms[0];
    let size = room.size();
    burrow.hallway.get(3) == A && size >= 3 && room.spaces(size - 3) != A
}

/// Mirror image situation to `deadlock_left` where a `D` amphipod could block others.
///
/// For example:
/// ```none
///     #############
///     #.......D...#
///     ###.#.#.#A###
///       #.#.#.#B#
///       #.#.#.#C#
///       #.#.#.#D#
///       #########
/// ```
///
/// The hallway has room for the top two amphipods but the `D` prevents the bottom two
/// from returning to their home burrow.
fn deadlock_right(burrow: &Burrow) -> bool {
    let room = &burrow.rooms[3];
    let size = room.size();
    burrow.hallway.get(7) == D && size >= 3 && room.spaces(size - 3) != D
}

/// Detects situation where amphipods in the hallway need to move past each other but
/// mutually block any further progress.
///
/// For example:
/// ```none
///     #############
///     #.....D.A...#
///     ###.#.#.#.###
///       #.#.#.#.#
///       #.#.#C#.#
///       #.#.#C#.#
///       #########
/// ```
///
/// In this situation, neither `A` nor `D` can move into `C`'s room but also block each other
/// from returning to their home burrow.
///
/// Another example:
/// ```none
///     #############
///     #.....C.A...#
///     ###.#.#.#.###
///       #.#.#.#.#
///       #.#.#B#.#
///       #.#.#C#.#
///       #########
/// ```
/// In this situation `C` blocks `A` from returning to its home burrow and `B` is also blocked
/// from moving out of the way.
fn deadlock_room(burrow: &Burrow, kind: usize) -> bool {
    let left_kind = burrow.hallway.get(1 + 2 * kind);
    let right_kind = burrow.hallway.get(3 + 2 * kind);

    left_kind != EMPTY
        && right_kind != EMPTY
        && left_kind >= kind
        && right_kind <= kind
        && !(burrow.rooms[kind].open(kind) && (kind == right_kind || kind == left_kind))
}
