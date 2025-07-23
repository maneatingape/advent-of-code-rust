//! # RPG Simulator 20XX
//!
//! The trick to get the outcome of each battle quickly is to divide the hero's health by the
//! boss's damage and vice-versa then find out how many turns each takes to win.
use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::Add;

#[derive(Clone, Copy)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Add for Item {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Item {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

type Result = (bool, u32);

pub fn parse(input: &str) -> Vec<Result> {
    let [boss_health, boss_damage, boss_armor]: [u32; 3] =
        input.iter_unsigned().chunk::<3>().next().unwrap();

    let weapon = [
        Item { cost: 8, damage: 4, armor: 0 },
        Item { cost: 10, damage: 5, armor: 0 },
        Item { cost: 25, damage: 6, armor: 0 },
        Item { cost: 40, damage: 7, armor: 0 },
        Item { cost: 74, damage: 8, armor: 0 },
    ];

    let armor = [
        Item { cost: 0, damage: 0, armor: 0 },
        Item { cost: 13, damage: 0, armor: 1 },
        Item { cost: 31, damage: 0, armor: 2 },
        Item { cost: 53, damage: 0, armor: 3 },
        Item { cost: 75, damage: 0, armor: 4 },
        Item { cost: 102, damage: 0, armor: 5 },
    ];

    let ring = [
        Item { cost: 25, damage: 1, armor: 0 },
        Item { cost: 50, damage: 2, armor: 0 },
        Item { cost: 100, damage: 3, armor: 0 },
        Item { cost: 20, damage: 0, armor: 1 },
        Item { cost: 40, damage: 0, armor: 2 },
        Item { cost: 80, damage: 0, armor: 3 },
    ];

    let mut combinations = Vec::with_capacity(22);
    combinations.push(Item { cost: 0, damage: 0, armor: 0 });

    for i in 0..6 {
        combinations.push(ring[i]);
        for j in (i + 1)..6 {
            combinations.push(ring[i] + ring[j]);
        }
    }

    let mut results = Vec::with_capacity(660);

    for first in weapon {
        for second in armor {
            for &third in &combinations {
                let Item { cost, damage, armor } = first + second + third;

                let hero_hit = damage.saturating_sub(boss_armor).max(1);
                let hero_turns = boss_health.div_ceil(hero_hit);
                let boss_hit = boss_damage.saturating_sub(armor).max(1);
                let boss_turns = 100_u32.div_ceil(boss_hit);
                let win = hero_turns <= boss_turns;

                results.push((win, cost));
            }
        }
    }

    results
}

pub fn part1(input: &[Result]) -> u32 {
    *input.iter().filter_map(|(w, c)| w.then_some(c)).min().unwrap()
}

pub fn part2(input: &[Result]) -> u32 {
    *input.iter().filter_map(|(w, c)| (!w).then_some(c)).max().unwrap()
}
