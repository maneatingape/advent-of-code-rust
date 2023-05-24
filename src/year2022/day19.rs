use crate::util::parse::*;
use std::ops::{Add, Mul, Sub};

const ZERO: Resources = Resources(0);
const ORE_BOT: Resources = Resources(1 << 24);
const CLAY_BOT: Resources = Resources(1 << 16);
const OBSIDIAN_BOT: Resources = Resources(1 << 8);
const GEODE_BOT: Resources = Resources(1);

#[derive(Clone, Copy)]
pub struct Resources(u32);

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Resources {
        Resources(self.0 + rhs.0)
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Resources) -> Resources {
        Resources(self.0 - rhs.0)
    }
}

impl Mul<u32> for Resources {
    type Output = Resources;

    fn mul(self, rhs: u32) -> Resources {
        Resources(self.0 * rhs)
    }
}

impl Resources {
    pub fn less_than_equal(self, other: &Self) -> bool {
        self.ore() <= other.ore()
            && self.clay() <= other.clay()
            && self.obsidian() <= other.obsidian()
            && self.geode() <= other.geode()
    }

    pub fn ore(self) -> u32 {
        (self.0 >> 24) & 0xff
    }

    pub fn clay(self) -> u32 {
        (self.0 >> 16) & 0xff
    }

    pub fn obsidian(self) -> u32 {
        (self.0 >> 8) & 0xff
    }

    pub fn geode(self) -> u32 {
        self.0 & 0xff
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
            ore_bot_cost: Resources(ore1 << 24),
            clay_bot_cost: Resources(ore2 << 24),
            obsidian_bot_cost: Resources((ore3 << 24) + (clay << 16)),
            geode_bot_cost: Resources((ore4 << 24) + (obsidian << 8)),
        }
    }
}

pub fn parse(input: &str) -> Vec<Blueprint> {
    input.iter_unsigned().collect::<Vec<u32>>().chunks_exact(7).map(Blueprint::parse).collect()
}

pub fn part1(input: &[Blueprint]) -> u32 {
    input.iter().map(|blueprint| blueprint.id * maximize(blueprint, 24, ORE_BOT, ZERO, 0)).sum()
}

pub fn part2(input: &[Blueprint]) -> u32 {
    input.iter().take(3).map(|blueprint| maximize(blueprint, 32, ORE_BOT, ZERO, 0)).product()
}

fn maximize(
    blueprint: &Blueprint,
    time: u32,
    bots: Resources,
    resources: Resources,
    geodes: u32,
) -> u32 {
    let baseline = resources.geode() + bots.geode() * time;
    let mut geodes = geodes.max(baseline);

    // Simple pruning
    let need_geode = time > 1 && {
        let n = time - 1;
        let extra = (n * (n + 1)) / 2;
        baseline + extra > geodes
    };

    let need_obsidian = need_geode
        && bots.obsidian() < blueprint.max_obsidian
        && time > 3
        && resources.obsidian() < (blueprint.max_obsidian - bots.obsidian()) * (time - 3);

    let need_clay = need_obsidian
        && bots.clay() < blueprint.max_clay
        && time > 5
        && resources.clay() < (blueprint.max_clay - bots.clay()) * (time - 5);

    let need_ore = need_geode
        && bots.ore() < blueprint.max_ore
        && time > 3
        && (resources.ore() < (blueprint.max_ore - bots.ore()) * (time - 3));

    if need_geode && bots.obsidian() > 0 {
        let result =
            next(blueprint, time, bots, resources, geodes, GEODE_BOT, blueprint.geode_bot_cost);
        geodes = geodes.max(result);
    }

    if need_obsidian && bots.clay() > 0 {
        let result = next(
            blueprint,
            time,
            bots,
            resources,
            geodes,
            OBSIDIAN_BOT,
            blueprint.obsidian_bot_cost,
        );
        geodes = geodes.max(result);
    }

    if need_clay {
        let result =
            next(blueprint, time, bots, resources, geodes, CLAY_BOT, blueprint.clay_bot_cost);
        geodes = geodes.max(result);
    }

    if need_ore {
        let result =
            next(blueprint, time, bots, resources, geodes, ORE_BOT, blueprint.ore_bot_cost);
        geodes = geodes.max(result);
    }

    geodes
}

fn next(
    blueprint: &Blueprint,
    time: u32,
    bots: Resources,
    resources: Resources,
    geodes: u32,
    bot: Resources,
    cost: Resources,
) -> u32 {
    for jump in 0..(time - 1) {
        let next = resources + bots * jump;
        if cost.less_than_equal(&next) {
            return maximize(blueprint, time - jump - 1, bots + bot, next + bots - cost, geodes);
        }
    }

    0
}
