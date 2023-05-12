use crate::util::hash::*;
use kind::*;

struct PriorityQueue<T> {
    todo: Vec<Vec<T>>,
    head: usize,
}

impl<T: Clone> PriorityQueue<T> {
    fn with_capacity(capacity: usize) -> PriorityQueue<T> {
        PriorityQueue { todo: vec![vec![]; capacity], head: 0 }
    }

    fn pop(&mut self) -> T {
        while self.todo[self.head].is_empty() {
            self.head += 1;
        }
        self.todo[self.head].pop().unwrap()
    }

    fn push(&mut self, priority: u32, state: T) {
        self.head = self.head.min(priority as usize);
        self.todo[priority as usize].push(state);
    }
}

mod kind {
    pub const A: u8 = 0;
    pub const B: u8 = 1;
    pub const C: u8 = 2;
    pub const D: u8 = 3;
    pub const ROOM: u8 = 4;
    pub const EMPTY: u8 = 5;

    pub const COST: [usize; 4] = [1, 10, 100, 1000];

    pub fn from(ascii: u8) -> u8 {
        ascii - b'A'
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Room {
    open: bool,
    size: u8,
    spaces: [u8; 4],
}

impl Room {
    fn new(spaces: [u8; 4]) -> Room {
        Room { open: false, size: 4, spaces }
    }

    fn peek(&self) -> usize {
        self.spaces[(self.size - 1) as usize] as usize
    }

    fn pop(&mut self, kind: usize) -> usize {
        self.size -= 1;
        let size = self.size as usize;
        let pod = self.spaces[size];
        self.spaces[size] = EMPTY;
        self.open = self.spaces.iter().take(size).all(|&p| p == kind as u8);
        pod as usize
    }

    fn push(&mut self, kind: usize) {
        self.spaces[self.size as usize] = kind as u8;
        self.size += 1;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Burrow {
    hallway: [u8; 12], // Faster to hash with extra element
    rooms: [Room; 4],
}

impl Burrow {
    fn new(room_a: [u8; 4], room_b: [u8; 4], room_c: [u8; 4], room_d: [u8; 4]) -> Burrow {
        Burrow {
            hallway: [
                EMPTY, EMPTY, ROOM, EMPTY, ROOM, EMPTY, ROOM, EMPTY, ROOM, EMPTY, EMPTY, EMPTY,
            ],
            rooms: [Room::new(room_a), Room::new(room_b), Room::new(room_c), Room::new(room_d)],
        }
    }
}

#[derive(Copy, Clone)]
struct State {
    energy: u32,
    heuristic: u32,
    burrow: Burrow,
}

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    let burrow = Burrow::new(
        [A, A, from(input[45]), from(input[31])],
        [B, B, from(input[47]), from(input[33])],
        [C, C, from(input[49]), from(input[35])],
        [D, D, from(input[51]), from(input[37])],
    );
    organize(burrow)
}

pub fn part2(input: &[u8]) -> u32 {
    let burrow = Burrow::new(
        [from(input[45]), D, D, from(input[31])],
        [from(input[47]), B, C, from(input[33])],
        [from(input[49]), A, B, from(input[35])],
        [from(input[51]), C, A, from(input[37])],
    );
    organize(burrow)
}

fn organize(burrow: Burrow) -> u32 {
    let mut seen = FastMapBuilder::empty();
    let mut todo = PriorityQueue::with_capacity(100_000);

    seen.insert(burrow, 0);
    todo.push(0, State { energy: 0, heuristic: heuristic(&burrow), burrow });

    loop {
        let state @ State { energy, heuristic, mut burrow } = todo.pop();
        let mut delta = 0;

        for i in 0..4 {
            if burrow.rooms[i].open && burrow.rooms[i].size < 4 {
                let offset = 2 + 2 * i;
                let forward = (offset + 1)..11;
                let reverse = (0..offset).rev();
                delta += condense(&mut burrow, i, forward);
                delta += condense(&mut burrow, i, reverse);
            }
        }

        if delta == 0 {
            for i in 0..4 {
                if !burrow.rooms[i].open {
                    let offset = 2 + 2 * i;
                    let forward = (offset + 1)..11;
                    let reverse = (0..offset).rev();
                    expand(&mut todo, &mut seen, &state, i, forward);
                    expand(&mut todo, &mut seen, &state, i, reverse);
                }
            }
        } else {
            let next_heuristic = heuristic - delta;
            let next_energy = energy + delta;

            if burrow.rooms.iter().all(|r| r.open && r.size == 4) {
                return next_energy;
            }

            let min = seen.get(&burrow).unwrap_or(&u32::MAX);
            if next_energy < *min {
                let next_state = State { energy: next_energy, heuristic: next_heuristic, burrow };
                seen.insert(burrow, next_energy);
                todo.push(next_energy + next_heuristic, next_state);
            }
        }
    }
}

fn heuristic(burrow: &Burrow) -> u32 {
    let mut distance = 0;
    let mut need_to_move = [0; 4];

    for (hallway_index, &kind) in burrow.hallway.iter().enumerate() {
        match kind {
            ROOM | EMPTY => (),
            _ => {
                let kind = kind as usize;
                let room_index = 2 + 2 * kind;
                let horizontal = hallway_index.abs_diff(room_index);
                need_to_move[kind] += 1;
                distance += COST[kind] * (need_to_move[kind] + horizontal);
            }
        }
    }

    for (original_kind, room) in burrow.rooms.iter().enumerate() {
        let mut blocker = false;

        for (depth, &kind) in room.spaces.iter().take(room.size as usize).enumerate() {
            let kind = kind as usize;
            if kind != original_kind {
                blocker = true;
                need_to_move[kind] += 1;
                let up = 4 - depth;
                let across = 2 * kind.abs_diff(original_kind);
                let down = need_to_move[kind];
                distance += COST[kind] * (up + across + down);
            } else if blocker {
                need_to_move[kind] += 1;
                let up = 4 - depth;
                let across = 2; // Nearest spot then back
                let down = need_to_move[kind];
                distance += COST[kind] * (up + across + down);
            }
        }
    }

    distance as u32
}

fn condense(burrow: &mut Burrow, kind: usize, iter: impl Iterator<Item = usize>) -> u32 {
    let dest_hallway_index = 2 + 2 * kind;
    let mut delta = 0;

    for hallway_index in iter {
        match burrow.hallway[hallway_index] {
            EMPTY => (),
            ROOM => {
                let room_index = (hallway_index - 2) / 2;

                while burrow.rooms[room_index].size > 0 && burrow.rooms[room_index].peek() == kind {
                    burrow.rooms[room_index].pop(room_index);
                    let up = 4 - burrow.rooms[room_index].size as usize;
                    let across = 2 * room_index.abs_diff(kind);
                    let down = 4 - burrow.rooms[kind].size as usize;
                    delta += COST[kind] * (up + across + down);
                    burrow.rooms[kind].push(kind);
                }
            }
            pod if pod as usize == kind => {
                burrow.hallway[hallway_index] = EMPTY;
                let across = hallway_index.abs_diff(dest_hallway_index);
                let down = 4 - burrow.rooms[kind].size as usize;
                delta += COST[kind] * (across + down);
                burrow.rooms[kind].push(kind);
            }
            _ => break,
        }
    }

    delta as u32
}

fn expand(
    todo: &mut PriorityQueue<State>,
    seen: &mut FastMap<Burrow, u32>,
    state: &State,
    room_index: usize,
    iter: impl Iterator<Item = usize>,
) {
    let State { energy, heuristic, mut burrow } = state;
    let kind = burrow.rooms[room_index].pop(room_index);

    for hallway_index in iter {
        match burrow.hallway[hallway_index] {
            ROOM => (),
            EMPTY => {
                let mut next = burrow;
                next.hallway[hallway_index] = kind as u8;

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

                let up = 4 - next.rooms[room_index].size as usize;
                let across = hallway_index.abs_diff(start);
                let delta = (COST[kind] * (up + across)) as u32;
                let next_energy = energy + delta;
                let min = seen.get(&next).unwrap_or(&u32::MAX);

                if next_energy < *min {
                    let adjust = if start == end {
                        across - 1
                    } else {
                        let lower = start.min(end);
                        let upper = start.max(end);
                        lower.saturating_sub(hallway_index) + hallway_index.saturating_sub(upper)
                    };

                    let extra = COST[kind] * 2 * adjust;
                    let next_heuristic = heuristic - delta + extra as u32;
                    let next_state =
                        State { energy: next_energy, heuristic: next_heuristic, burrow: next };

                    seen.insert(next, next_energy);
                    todo.push(next_energy + next_heuristic, next_state);
                }
            }
            _ => break,
        }
    }
}

fn deadlock_left(burrow: &Burrow) -> bool {
    let room = &burrow.rooms[0];
    let size = room.size as usize;
    burrow.hallway[3] == A && room.size >= 3 && room.spaces[size - 3] != A
}

fn deadlock_right(burrow: &Burrow) -> bool {
    let room = &burrow.rooms[3];
    let size = room.size as usize;
    burrow.hallway[7] == D && room.size >= 3 && room.spaces[size - 3] != D
}

fn deadlock_room(burrow: &Burrow, kind: u8) -> bool {
    let left_kind = burrow.hallway[1 + 2 * kind as usize];
    let right_kind = burrow.hallway[3 + 2 * kind as usize];

    left_kind != EMPTY
        && right_kind != EMPTY
        && left_kind >= kind
        && right_kind <= kind
        && !(burrow.rooms[kind as usize].open && (kind == right_kind || kind == left_kind))
}
