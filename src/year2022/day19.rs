//! # Not Enough Minerals
//!
//! The solution is [branch and bound](https://en.wikipedia.org/wiki/Branch_and_bound) using
//! a [depth first search](https://en.wikipedia.org/wiki/Depth-first_search) to enumerate every
//! possible combination combined with heuristics to prune those combinations in order to achieve
//! a reasonable running time.
//!
//! The most import heuristic is:
//! * Assume ore is infinite.
//! * Always build a clay robot.
//! * Check if we can do better than the highest score so far in the remaining time, building
//!   only geode or obsidian bots.
//!
//! As these simplified rules will always score higher than the real rules, we can immediately
//! prune any branch that can't possibly exceed the current high score.
//!
//! The second helpful heuristic is:
//! * Don't build more bots for a particular mineral than the maximum possible consumption in
//!   a single turn.
//!
//! As we can only build one bot per turn, we will never need to generate more resources than
//! that bot can use. For example, if ore robots need 2 ore, clay robots 3 ore, obsidian robots
//! 4 ore and geode robots 5 ore, then the most possible ore robots that we need to build is 5.
//! Any more would go to waste. The same applies for clay and obsidian.
//!
//! The third helpful heuristic is:
//! * Don't build any robot during the last minute
//! * Don't build ore or obsidian robots during the last 3 minutes.
//! * Don't build clay robots during the last 5 minutes.
//!
//! Building any robot during the last minute means that it will be ready *after* the time runs
//! out so it will contribute nothing. The other two rules are corollaries of this rule.
//!
//! For example say we build an obsidian robot with 3 minutes left. It will be ready and collect
//! a resource with two minutes left, which can be spent on a geode robot with 1 minute left,
//! which is too late.
//!
//! Since we only need clay for obsidian robots it doesn't make sense to build clay robots less
//! than two minutes before the cutoff for obsidian robots.
//!
//! The final important optimization is that we don't increment minute by minute. Instead once
//! we decide to build a robot of a particular type, we "fast forward" in time until there are
//! enough resources to build that robot. This cuts down on a lot of duplicate intermediate states.
use crate::util::parse::*;
use std::ops::{Add, Sub};

/// Each robot generates 1 mineral of a particular type.
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

    /// This is used to compare robot costs so we don't need to check geodes.
    fn less_than_equal(self, rhs: Self) -> bool {
        self.ore <= rhs.ore && self.clay <= rhs.clay && self.obsidian <= rhs.obsidian
    }
}

/// Implement operators so that we can use `+` and `-` notation to add and subtract minerals.
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

/// Depth first search over every possible combination pruning branches using heuristics.
fn dfs(blueprint: &Blueprint, result: &mut u32, time: u32, bots: Mineral, resources: Mineral) {
    // Extrapolate total geodes from the current state in the remaining time.
    *result = (*result).max(resources.geode + bots.geode * time);

    // Check if this state can improve on the existing high score.
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

/// Simplify the blueprints so that we only attempt to build either geode or obsidian robots,
/// then check that the estimated maximum possible score is greater than the current high score.
/// Additionally we always build a clay robot each turn.
///
/// Since this will always score higher, so we can immediately prune any branch that can't
/// possibly beat the high score.
#[inline]
fn heuristic(
    blueprint: &Blueprint,
    result: u32,
    time: u32,
    mut bots: Mineral,
    mut resources: Mineral,
) -> bool {
    for _ in 0..time {
        // Assume ore is infinite.
        resources.ore = blueprint.max_ore;

        // Only attempt to build geode or obsidian robots.
        if blueprint.geode_cost.less_than_equal(resources) {
            resources = resources + bots - blueprint.geode_cost;
            bots = bots + GEODE_BOT;
        } else if blueprint.obsidian_cost.less_than_equal(resources) {
            resources = resources + bots - blueprint.obsidian_cost;
            bots = bots + OBSIDIAN_BOT;
        } else {
            resources = resources + bots;
        }

        // Always build a clay bot.
        bots = bots + CLAY_BOT;
    }

    resources.geode > result
}

/// "Fast forward" in time until we can build a robot of a particular type. This could possibly
/// by the next minute if we already have enough resources.
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
