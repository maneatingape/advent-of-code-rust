//! # Radioisotope Thermoelectric Generators
//!
//! Solves using a [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) from the
//! starting position where each next state is the possible elevator moves either one floor up or
//! down. This was faster than using [A*](https://en.wikipedia.org/wiki/A*_search_algorithm)
//! with a heuristic.
//!
//! A huge critical optimization is the observation that generator and chip pairs are *fungible*.
//! A configuration that starts with two pairs on floor one takes the same number of steps to
//! solve whether pair A or pair B is moved first (that is, the setup [-;-;AG,AM;BG,BM] while on
//! floor 2 is indistinguishible from [-;-;BG,BM;AG,AM] on floor 2 in terms of the final result).
//! However, the relative positions of pairs still matters (the setup [AM;AG;BG;BM] on floor two
//! can move BG up or down; but the setup [AM;BG;AG;BM] on floor two can only move AG up).  To
//! maximize state sharing, represent each pair's generator and microchip position as hex
//! digits, but merge all permutations by sorting those hex digit pairs during the hash
//! function.  Including the elevator position, the hash value requires up to 30 useful bits
//! (2 + 7*4) if densely packed, although this uses a 64-bit struct.
//!
//! Next, observe that adding a chip and generator pair on floor 1 adds 12 moves to the final
//! solution; likewise, removing a pair from floor 1 (but only if there is still something
//! else left on the floor) can be solved in 12 fewer moves.  Tracking a smaller number of
//! chip and generator pairs, then adjusting by the 12 times the number of ignored pairs,
//! is inherently faster.
//!
//! The rules for a valid floor are either:
//!
//! * Any amount of microchips only with no generators.
//! * Any microchip on a floor with at least one generator must have its own generator on that floor.
//!
//! This allows us to efficiently memoize previously seen states and reject any that we've seen
//! before extremely quickly. Other optimizations:
//!
//! * If we can move 2 items up, then skip only moving 1 item up.
//! * If we can move 1 item down, then skip moving 2 items down.
//! * Moving a microchip and generator together is only safe if they are the same type.
//! * If floor 1 is empty then don't move items back down to it, similarly if both floor 1 and
//!   floor 2 are empty then don't move items to them.
use crate::util::bitset::*;
use crate::util::hash::*;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

// A one-hot encoding is more efficient than 0-3.
const FLOOR1: u8 = 1 << 0;
const FLOOR2: u8 = 1 << 1;
const FLOOR3: u8 = 1 << 2;
const FLOOR4: u8 = 1 << 3;

// The size of the struct is most efficient at 8 bytes, even when we optimize by not tracking
// pairs that start on floor 1.  Sorting u8 is easier than sorting four-bit nibbles, so reserve
// space for up to each of the 5 pairs of part 1.  Part 2 would add another two pairs, but since
// we know their impact is constant, we can instead store other useful data there, that gets
// ignored when computing the manual hash and equality of the struct.  As such, this type
// relies on an invariant that only a State passed through canon() may be compared/hashed.
#[derive(Clone, Copy, Default, Eq)]
pub struct State {
    elevator: u8, // One-hot encoding with FLOORn constants.
    steps: u8,
    in_use: u8, // How many entries of pairs matter
    pairs: [Pair; 5],
}

// Intentionally ignore the fields not relevant to caching; assumes pairs is sorted.
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator.hash(state);
        self.pairs.hash(state);
    }
}

// Intentionally ignore the fields not relevant to caching; assumes pairs is sorted.
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.elevator == other.elevator && self.in_use == other.in_use && self.pairs == other.pairs
    }
}

impl State {
    // Return a bitmask of generators on the given floor.
    fn generators(self, floor: u8) -> u8 {
        let mut accum = 0;
        for (i, p) in self.pairs.iter().enumerate() {
            accum += u8::from(p.generator() == floor) << i;
        }
        accum
    }

    // Return a bitmask of microchips on the given floor.
    fn microchips(self, floor: u8) -> u8 {
        let mut accum = 0;
        for (i, p) in self.pairs.iter().enumerate() {
            accum += u8::from(p.microchip() == floor) << i;
        }
        accum
    }

    // Return a bitmask of all items on the given floor.
    fn items(self, floor: u8) -> u16 {
        let mut accum = 0;
        for (i, p) in self.pairs.iter().enumerate() {
            accum += (u16::from(p.generator() == floor) * 2 + u16::from(p.microchip() == floor))
                << (i * 2);
        }
        accum
    }

    // Critical optimization treating generators and microchips as fungible.
    fn canon(mut self) -> Option<State> {
        for i in 0..4 {
            let floor = 1 << i as u8;
            let gens = self.generators(floor);
            let chips = self.microchips(floor);
            // Reject any inconsistent setup.
            if gens != 0 && (chips & !gens) != 0 {
                return None;
            }
        }
        // Rearrange remaining pairs into canonical order.
        self.pairs.sort();
        Some(self)
    }

    // Attempt to adjust state by moving a single item up or down.
    fn move_one(self, up: bool, item: usize) -> Option<State> {
        // Build the new state
        let mut state = self;
        state.steps += 1;
        let index = item / 2;
        if up {
            state.pairs[index].floors += state.elevator << (4 * (item % 2)) as u8;
            state.elevator *= 2;
        } else {
            state.elevator /= 2;
            state.pairs[index].floors -= state.elevator << (4 * (item % 2)) as u8;
        }

        state.canon()
    }

    // Attempt to adjust state by moving two distinct items up or down.
    fn move_two(self, up: bool, item1: usize, item2: usize) -> Option<State> {
        // Don't mix-and-match microchip and generator from distinct pairs.
        if item1.is_multiple_of(2) != item2.is_multiple_of(2) && item1 / 2 != item2 / 2 {
            return None;
        }

        // Build the new state
        let mut state = self;
        state.steps += 1;
        let index1 = item1 / 2;
        let index2 = item2 / 2;
        if up {
            state.pairs[index1].floors += state.elevator << (4 * (item1 % 2)) as u8;
            state.pairs[index2].floors += state.elevator << (4 * (item2 % 2)) as u8;
            state.elevator *= 2;
        } else {
            state.elevator /= 2;
            state.pairs[index1].floors -= state.elevator << (4 * (item1 % 2)) as u8;
            state.pairs[index2].floors -= state.elevator << (4 * (item2 % 2)) as u8;
        }

        state.canon()
    }
}

// Store the one-hot encoding of the floors for the generator (upper half) and microchip (lower half)
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pair {
    floors: u8,
}

impl Pair {
    #[inline]
    fn generator(self) -> u8 {
        self.floors >> 4
    }

    #[inline]
    fn microchip(self) -> u8 {
        self.floors & 0xf
    }
}

pub fn parse(input: &str) -> u8 {
    let mut state = State { elevator: FLOOR1, ..State::default() };
    let mut pairs = FastMap::new();

    // Find all items, and set an entry in state.pairs for each element name
    let mut floor = 0;
    let words: Vec<_> = input.split_whitespace().collect();

    for w in words.windows(2) {
        if w[0] == "The" {
            floor = if floor == 0 { FLOOR1 } else { floor * 2 };
        } else if w[0] == "a" {
            if let Some(prefix) = w[1].strip_suffix("-compatible") {
                *pairs.entry(prefix).or_insert(0) |= floor;
            } else {
                *pairs.entry(w[1]).or_insert(0) |= floor << 4;
            }
        }
    }

    state.in_use = pairs.len() as u8;
    for (i, floors) in pairs.into_values().enumerate() {
        state.pairs[i] = Pair { floors };
    }

    // Optimize search to ignore item pairs starting on floor 0
    for i in 0..state.pairs.len() {
        if state.pairs[i].floors == (FLOOR1 << 4) | FLOOR1 && state.items(FLOOR1).count_ones() > 2 {
            state.pairs[i].floors = 0;
            state.in_use -= 1;
            state.steps += 12;
        }
    }

    bfs(state.canon().unwrap())
}

pub fn part1(input: &u8) -> u8 {
    *input
}

pub fn part2(input: &u8) -> u8 {
    // Both pairs add 12 steps each
    *input + 24
}

fn bfs(start: State) -> u8 {
    let mut todo = VecDeque::new();
    let mut seen = FastSet::with_capacity(500);

    todo.push_back(start);
    seen.insert(start);

    while let Some(state) = todo.pop_front() {
        if state.elevator == FLOOR4 && state.items(FLOOR4).count_ones() == 2 * state.in_use as u32 {
            return state.steps;
        }

        // Iterate over items that can be moved.
        let items = state.items(state.elevator);
        let mut added = false;

        // When moving down, try move 1 first, use move 2 only if no move 1
        // Don't move down from 0, or down into empty 0 or 1
        if !(state.elevator == FLOOR1
            || (state.elevator == FLOOR2 && state.items(FLOOR1) == 0)
            || (state.elevator == FLOOR3 && state.items(FLOOR1) + state.items(FLOOR2) == 0))
        {
            for i in items.biterator() {
                if let Some(next) = state.move_one(false, i)
                    && seen.insert(next)
                {
                    added = true;
                    todo.push_back(next);
                }
            }
            if !added {
                for i in items.biterator() {
                    for j in (items & !((1 << (i + 1)) - 1)).biterator() {
                        if let Some(next) = state.move_two(false, i, j)
                            && seen.insert(next)
                        {
                            todo.push_back(next);
                        }
                    }
                }
            }
        }

        // When moving up, try move 2 first, use move 1 only if no move 2.
        // Don't move up from 3.
        if state.elevator != FLOOR4 {
            added = false;
            for i in items.biterator() {
                for j in (items & !((1 << (i + 1)) - 1)).biterator() {
                    if let Some(next) = state.move_two(true, i, j)
                        && seen.insert(next)
                    {
                        added = true;
                        todo.push_back(next);
                    }
                }
            }
            if !added {
                for i in items.biterator() {
                    if let Some(next) = state.move_one(true, i)
                        && seen.insert(next)
                    {
                        todo.push_back(next);
                    }
                }
            }
        }
    }

    unreachable!()
}
