use crate::util::hash::*;
use std::array::from_fn;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::*;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const ROOM: usize = 4;
const EMPTY: usize = 5;
const COST: [usize; 4] = [1, 10, 100, 1000];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Room {
    packed: u16,
}

impl Room {
    fn new(spaces: [usize; 4]) -> Room {
        let packed = (1 << 12) | (spaces[0] << 9) | (spaces[1] << 6) | (spaces[2] << 3) | spaces[3];
        Room { packed: packed as u16 }
    }

    fn size(self) -> usize {
        ((15 - self.packed.leading_zeros()) / 3) as usize
    }

    fn peek(self) -> Option<usize> {
        (self.packed > 1).then_some((self.packed & 0b111) as usize)
    }

    fn pop(&mut self) -> usize {
        let pod = (self.packed & 0b111) as usize;
        self.packed >>= 3;
        pod
    }

    fn open(self, kind: usize) -> bool {
        self.packed == 1
            || self.packed == (1 << 3) + (kind as u16)
            || self.packed == (1 << 6) + (kind as u16 * 9)
            || self.packed == (1 << 9) + (kind as u16 * 73)
            || self.packed == (1 << 12) + (kind as u16 * 585)
    }

    fn push(&mut self, kind: usize) {
        self.packed = (self.packed << 3) | (kind as u16);
    }

    fn spaces(self, index: usize) -> usize {
        let adjusted = 3 * (self.size() - 1 - index);
        ((self.packed >> adjusted) & 0b111) as usize
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Hallway {
    packed: usize,
}

impl Hallway {
    fn new() -> Hallway {
        Hallway { packed: 0x55454545455 }
    }

    fn get(self, index: usize) -> usize {
        (self.packed >> (index * 4)) & 0xf
    }

    fn set(&mut self, index: usize, value: usize) {
        let mask = !(0xf << (index * 4));
        let value = value << (index * 4);
        self.packed = (self.packed & mask) | value;
    }
}

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

#[derive(Copy, Clone, PartialEq, Eq)]
struct State {
    burrow: Burrow,
    energy: usize,
}

impl PartialOrd for State {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b.saturating_sub(b'A') as usize).collect())
        .collect()
}

pub fn part1(input: &[Vec<usize>]) -> usize {
    let burrow = Burrow::new([
        [A, A, input[3][3], input[2][3]],
        [B, B, input[3][5], input[2][5]],
        [C, C, input[3][7], input[2][7]],
        [D, D, input[3][9], input[2][9]],
    ]);
    organize(burrow)
}

pub fn part2(input: &[Vec<usize>]) -> usize {
    let burrow = Burrow::new([
        [input[3][3], D, D, input[2][3]],
        [input[3][5], B, C, input[2][5]],
        [input[3][7], A, B, input[2][7]],
        [input[3][9], C, A, input[2][9]],
    ]);
    organize(burrow)
}

fn organize(burrow: Burrow) -> usize {
    let mut todo = BinaryHeap::with_capacity(20_000);
    let mut seen = FastMap::with_capacity(20_000);

    todo.push(State { burrow, energy: best_possible(&burrow) });

    while let Some(state) = todo.pop() {
        let State { mut burrow, energy } = state;
        let open: [bool; 4] = from_fn(|i| burrow.rooms[i].open(i));

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
            if burrow.rooms.iter().enumerate().all(|(i, r)| open[i] && r.size() == 4) {
                return energy;
            }

            // Moving back to home burrow does not change cost
            let min = seen.get(&burrow).unwrap_or(&usize::MAX);
            if energy < *min {
                todo.push(State { burrow, energy });
                seen.insert(burrow, energy);
            }
        } else {
            for (i, &open) in open.iter().enumerate() {
                if !open {
                    let offset = 2 + 2 * i;
                    let forward = (offset + 1)..11;
                    let reverse = (0..offset).rev();
                    expand(&mut todo, &mut seen, &state, i, forward);
                    expand(&mut todo, &mut seen, &state, i, reverse);
                }
            }
        }
    }

    unreachable!()
}

fn best_possible(burrow: &Burrow) -> usize {
    let mut energy = 0;
    let mut need_to_move = [0; 4];

    for (original_kind, room) in burrow.rooms.iter().enumerate() {
        let mut blocker = false;

        for depth in 0..room.size() {
            let kind = room.spaces(depth);
            if kind != original_kind {
                blocker = true;
                need_to_move[kind] += 1;
                let up = 4 - depth;
                let across = 2 * kind.abs_diff(original_kind);
                let down = need_to_move[kind];
                energy += COST[kind] * (up + across + down);
            } else if blocker {
                need_to_move[kind] += 1;
                let up = 4 - depth;
                let across = 2; // Nearest spot then back
                let down = need_to_move[kind];
                energy += COST[kind] * (up + across + down);
            }
        }
    }

    energy
}

fn condense(burrow: &mut Burrow, kind: usize, iter: impl Iterator<Item = usize>) -> bool {
    let mut changed = false;

    for hallway_index in iter {
        match burrow.hallway.get(hallway_index) {
            EMPTY => (),
            ROOM => {
                let room_index = (hallway_index - 2) / 2;

                while burrow.rooms[room_index].peek() == Some(kind) {
                    burrow.rooms[room_index].pop();
                    burrow.rooms[kind].push(kind);
                    changed = true;
                }
            }
            pod if pod == kind => {
                burrow.hallway.set(hallway_index, EMPTY);
                burrow.rooms[kind].push(kind);
                changed = true;
            }
            _ => break,
        }
    }

    changed
}

fn expand(
    todo: &mut BinaryHeap<State>,
    seen: &mut FastMap<Burrow, usize>,
    state: &State,
    room_index: usize,
    iter: impl Iterator<Item = usize>,
) {
    let State { mut burrow, energy } = state;
    let kind = burrow.rooms[room_index].pop();

    for hallway_index in iter {
        match burrow.hallway.get(hallway_index) {
            ROOM => (),
            EMPTY => {
                let mut next = burrow;
                next.hallway.set(hallway_index, kind);

                if deadlock_left(&next)
                    || deadlock_right(&next)
                    || deadlock_room(&next, 0)
                    || deadlock_room(&next, 1)
                    || deadlock_room(&next, 2)
                    || deadlock_room(&next, 3)
                {
                    continue;
                }

                let start = 2 + 2 * room_index;
                let end = 2 + 2 * kind;

                let adjust = if start == end {
                    let across = hallway_index.abs_diff(start);
                    across - 1
                } else {
                    let lower = start.min(end);
                    let upper = start.max(end);
                    lower.saturating_sub(hallway_index) + hallway_index.saturating_sub(upper)
                };

                let extra = COST[kind] * 2 * adjust;

                // Key optimization.
                // If not our home burrow then must move out of the way.
                if kind != room_index && extra == 0 {
                    continue;
                }

                let next_energy = energy + extra;
                let min = seen.get(&next).unwrap_or(&usize::MAX);

                if next_energy < *min {
                    todo.push(State { burrow: next, energy: next_energy });
                    seen.insert(next, next_energy);
                }
            }
            _ => break,
        }
    }
}

fn deadlock_left(burrow: &Burrow) -> bool {
    let room = &burrow.rooms[0];
    let size = room.size();
    burrow.hallway.get(3) == A && size >= 3 && room.spaces(size - 3) != A
}

fn deadlock_right(burrow: &Burrow) -> bool {
    let room = &burrow.rooms[3];
    let size = room.size();
    burrow.hallway.get(7) == D && size >= 3 && room.spaces(size - 3) != D
}

fn deadlock_room(burrow: &Burrow, kind: usize) -> bool {
    let left_kind = burrow.hallway.get(1 + 2 * kind);
    let right_kind = burrow.hallway.get(3 + 2 * kind);

    left_kind != EMPTY
        && right_kind != EMPTY
        && left_kind >= kind
        && right_kind <= kind
        && !(burrow.rooms[kind].open(kind) && (kind == right_kind || kind == left_kind))
}
