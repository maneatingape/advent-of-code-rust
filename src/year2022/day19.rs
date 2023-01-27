use crate::util::parse::*;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::{Add, Mul, Sub};

const ZERO: Resources = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };
const ORE_BOT: Resources = Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 };
const CLAY_BOT: Resources = Resources { ore: 0, clay: 1, obsidian: 0, geode: 0 };
const OBSIDIAN_BOT: Resources = Resources { ore: 0, clay: 0, obsidian: 1, geode: 0 };
const GEODE_BOT: Resources = Resources { ore: 0, clay: 0, obsidian: 0, geode: 1 };

#[derive(Clone, Copy, PartialEq)]
pub struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Resources {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Resources) -> Resources {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Mul<u32> for Resources {
    type Output = Resources;

    fn mul(self, rhs: u32) -> Resources {
        Resources {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        {
            self.ore > other.ore
            || self.clay > other.clay
            || self.obsidian > other.obsidian
            || self.geode > other.geode
        }
        .then_some(Ordering::Greater)
    }
}

pub struct Blueprint {
    id: u32,
    max_ore: u32,
    max_clay: u32,
    max_obsidian: u32,
    ore_bot_cost: Resources,
    clay_bot_cost: Resources,
    obsidian_bot_cost: Resources,
    geode_bot_cost: Resources,
}

impl Blueprint {
    fn parse(chunk: &[u32]) -> Blueprint {
        let id = chunk[0];
        let ore1 = chunk[1];
        let ore2 = chunk[2];
        let ore3 = chunk[3];
        let clay = chunk[4];
        let ore4 = chunk[5];
        let obsidian = chunk[6];

        Blueprint {
            id,
            max_ore: ore1.max(ore2).max(ore3).max(ore4),
            max_clay: clay,
            max_obsidian: obsidian,
            ore_bot_cost: Resources { ore: ore1, clay: 0, obsidian: 0, geode: 0 },
            clay_bot_cost: Resources { ore: ore2, clay: 0, obsidian: 0, geode: 0 },
            obsidian_bot_cost: Resources { ore: ore3, clay: clay, obsidian: 0, geode: 0 },
            geode_bot_cost: Resources { ore: ore4, clay: 0, obsidian: obsidian, geode: 0 },
        }
    }
}

struct State {
    time: u32,
    bots: Resources,
    resources: Resources,
}

pub fn parse(input: &str) -> Vec<Blueprint> {
    input
        .to_unsigned_iter()
        .collect::<Vec<u32>>()
        .chunks_exact(7)
        .map(Blueprint::parse)
        .collect()
}

pub fn part1(input: &[Blueprint]) -> u32 {
    input
        .iter()
        .map(|blueprint| blueprint.id * maximize(blueprint, 24))
        .sum()
}

pub fn part2(input: &[Blueprint]) -> u32 {
    input
        .iter()
        .take(3)
        .map(|blueprint| maximize(blueprint, 32))
        .product()
}

fn maximize(blueprint: &Blueprint, time: u32) -> u32 {
    let start = State {
        time,
        bots: ORE_BOT,
        resources: ZERO,
    };
    let mut todo = VecDeque::from([start]);
    let mut result = 0;

    while let Some(state) = todo.pop_front() {
        let State { time, bots, resources } = state;

        let baseline = resources.geode + bots.geode * time;
        result = result.max(baseline);

        // Simple pruning
        let need_geode =
            time > 1
            && {
                let n = time - 1;
                let extra = (n * (n + 1)) / 2;
                baseline + extra > result
            };

        let need_obsidian =
            need_geode
            && bots.obsidian < blueprint.max_obsidian
            && time > 3
            && resources.obsidian < (blueprint.max_obsidian - bots.obsidian) * (time - 3);

        let need_clay =
            need_obsidian
            && bots.clay < blueprint.max_clay
            && time > 5
            && resources.clay < (blueprint.max_clay - bots.clay) * (time - 5);

        let need_ore =
            need_geode
            && bots.ore < blueprint.max_ore
            && time > 3
            && (resources.ore < (blueprint.max_ore - bots.ore) * (time - 3));

        if need_geode && bots.obsidian > 0 {
            push(&mut todo, &state, GEODE_BOT, blueprint.geode_bot_cost);
        }

        if need_obsidian && bots.clay > 0 {
            push(&mut todo, &state, OBSIDIAN_BOT, blueprint.obsidian_bot_cost);
        }

        if need_clay {
            push(&mut todo, &state, CLAY_BOT, blueprint.clay_bot_cost);
        }

        if need_ore {
            push(&mut todo, &state, ORE_BOT, blueprint.ore_bot_cost);
        }
    }

    result
}

fn push(todo: &mut VecDeque<State>, state: &State, bot: Resources, cost: Resources) {
    let State { time, bots, resources } = state;

    for jump in 0..(time - 1) {
        let next = *resources + *bots * jump;
        if !(cost > next) {
            let state = State {
                time: time - jump - 1,
                bots: *bots + bot,
                resources: next + *bots - cost,
            };
            todo.push_back(state);
            break;
        }
    }
}
