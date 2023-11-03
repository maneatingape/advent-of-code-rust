use crate::util::parse::*;
use std::ops::{Add, Sub};

const ZERO: Mineral = Mineral::from(0, 0, 0, 0);
const ORE_BOT: Mineral = Mineral::from(1, 0, 0, 0);
const CLAY_BOT: Mineral = Mineral::from(0, 1, 0, 0);
const OBSIDIAN_BOT: Mineral = Mineral::from(0, 0, 1, 0);
const GEODE_BOT: Mineral = Mineral::from(0, 0, 0, 1);

#[derive(Clone, Copy)]
struct Mineral {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Mineral {
    const fn from(ore: u32, clay: u32, obsidian: u32, geode: u32) -> Self {
        Mineral { ore, clay, obsidian, geode }
    }

    fn less_than_equal(self, rhs: Self) -> bool {
        self.ore <= rhs.ore && self.clay <= rhs.clay && self.obsidian <= rhs.obsidian
    }
}

impl Add for Mineral {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Mineral {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Mineral {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Mineral {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

pub struct Blueprint {
    id: u32,
    max_ore: u32,
    max_clay: u32,
    max_obsidian: u32,
    ore_cost: Mineral,
    clay_cost: Mineral,
    obsidian_cost: Mineral,
    geode_cost: Mineral,
}

impl Blueprint {
    fn from(chunk: [u32; 7]) -> Self {
        let [id, ore1, ore2, ore3, clay, ore4, obsidian] = chunk;
        Blueprint {
            id,
            max_ore: ore1.max(ore2).max(ore3).max(ore4),
            max_clay: clay,
            max_obsidian: obsidian,
            ore_cost: Mineral::from(ore1, 0, 0, 0),
            clay_cost: Mineral::from(ore2, 0, 0, 0),
            obsidian_cost: Mineral::from(ore3, clay, 0, 0),
            geode_cost: Mineral::from(ore4, 0, obsidian, 0),
        }
    }
}

pub fn parse(input: &str) -> Vec<Blueprint> {
    input
        .iter_unsigned()
        .collect::<Vec<_>>()
        .chunks_exact(7)
        .map(|slice| Blueprint::from(slice.try_into().unwrap()))
        .collect()
}

pub fn part1(input: &[Blueprint]) -> u32 {
    input.iter().map(|blueprint| blueprint.id * maximize(blueprint, 24)).sum()
}

pub fn part2(input: &[Blueprint]) -> u32 {
    input.iter().take(3).map(|blueprint| maximize(blueprint, 32)).product()
}

fn maximize(blueprint: &Blueprint, time: u32) -> u32 {
    let mut result = 0;
    dfs(blueprint, &mut result, time, ORE_BOT, ZERO);
    result
}

fn dfs(blueprint: &Blueprint, result: &mut u32, time: u32, bots: Mineral, resources: Mineral) {
    *result = (*result).max(resources.geode + bots.geode * time);

    if heuristic(blueprint, *result, time, bots, resources) {
        if bots.obsidian > 0 && time > 1 {
            next(blueprint, result, time, bots, resources, GEODE_BOT, blueprint.geode_cost);
        }
        if bots.obsidian < blueprint.max_obsidian && bots.clay > 0 && time > 3 {
            next(blueprint, result, time, bots, resources, OBSIDIAN_BOT, blueprint.obsidian_cost);
        }
        if bots.ore < blueprint.max_ore && time > 3 {
            next(blueprint, result, time, bots, resources, ORE_BOT, blueprint.ore_cost);
        }
        if bots.clay < blueprint.max_clay && time > 5 {
            next(blueprint, result, time, bots, resources, CLAY_BOT, blueprint.clay_cost);
        }
    }
}

#[inline]
fn heuristic(
    blueprint: &Blueprint,
    result: u32,
    time: u32,
    mut bots: Mineral,
    mut resources: Mineral,
) -> bool {
    for _ in 0..time {
        resources.ore = blueprint.max_ore;
        resources.clay = blueprint.max_clay;

        if blueprint.geode_cost.less_than_equal(resources) {
            resources = resources + bots - blueprint.geode_cost;
            bots = bots + GEODE_BOT;
        } else {
            resources = resources + bots - blueprint.obsidian_cost;
            bots = bots + OBSIDIAN_BOT;
        }
    }

    resources.geode > result
}

#[inline]
fn next(
    blueprint: &Blueprint,
    result: &mut u32,
    time: u32,
    bots: Mineral,
    mut resources: Mineral,
    new_bot: Mineral,
    cost: Mineral,
) {
    for jump in 1..time {
        if cost.less_than_equal(resources) {
            dfs(blueprint, result, time - jump, bots + new_bot, resources + bots - cost);
            break;
        }
        resources = resources + bots;
    }
}
