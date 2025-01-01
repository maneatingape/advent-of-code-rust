//! # Immune System Simulator 20XX
//!
//! Similar to [`Day 15`] we implement the rules precisely, paying attention to edge cases.
//!
//! In particular during part two, it's possible for a fight to end in a draw, if both armies
//! become too weak to destroy any further units. As each fight is independent, we find the
//! minimum boost value with a multithreaded parallel search.
//!
//! [`Day 15`]: crate::year2018::day15
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::thread::*;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::mpsc::{Sender, channel};

pub struct Input {
    immune: Vec<Group>,
    infection: Vec<Group>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Immune,
    Infection,
    Draw,
}

#[derive(Clone, Copy)]
struct Group {
    units: i32,
    hit_points: i32,
    damage: i32,
    initiative: i32,
    weak: u32,
    immune: u32,
    attack: u32,
    chosen: u32,
}

/// Convenience functions.
impl Group {
    fn effective_power(&self) -> i32 {
        self.units * self.damage
    }

    /// Attack types are stored as a bitmask for quick comparison.
    fn actual_damage(&self, other: &Group) -> i32 {
        if self.attack & other.weak != 0 {
            2 * self.effective_power()
        } else if self.attack & other.immune == 0 {
            self.effective_power()
        } else {
            0
        }
    }

    fn target_selection_order(&self) -> (i32, i32) {
        (-self.effective_power(), -self.initiative)
    }

    fn attack(&self, defender: &mut Self) -> i32 {
        // Clamp damage to 0 as units may be negative,
        // if this unit was wiped out in an earlier attack.
        let damage = self.actual_damage(defender).max(0);
        let amount = damage / defender.hit_points;
        defender.units -= amount;
        amount
    }
}

/// Shared between threads for part two.
struct Shared {
    done: AtomicBool,
    boost: AtomicI32,
    tx: Sender<(i32, i32)>,
}

pub fn parse<'a>(input: &'a str) -> Input {
    // Use a bitmask to store each possible attack type.
    let mut elements = FastMap::new();
    let mut mask = |key: &'a str| {
        let next = 1 << elements.len();
        *elements.entry(key).or_insert(next)
    };

    let (first, second) = input.split_once("\n\n").unwrap();
    let immune = parse_group(first, &mut mask);
    let infection = parse_group(second, &mut mask);
    Input { immune, infection }
}

pub fn part1(input: &Input) -> i32 {
    let (_, units) = fight(input, 0);
    units
}

pub fn part2(input: &Input) -> i32 {
    let (tx, rx) = channel();
    let shared = Shared { done: AtomicBool::new(false), boost: AtomicI32::new(1), tx };

    // Use as many cores as possible to parallelize the search.
    spawn(|| worker(input, &shared));

    // Hang up the channel.
    drop(shared.tx);
    // Find lowest possible power.
    rx.iter().min_by_key(|&(boost, _)| boost).map(|(_, units)| units).unwrap()
}

fn worker(input: &Input, shared: &Shared) {
    while !shared.done.load(Ordering::Relaxed) {
        // Get the next attack boost, incrementing it atomically for the next fight.
        let boost = shared.boost.fetch_add(1, Ordering::Relaxed);

        // If the reindeer wins then set the score and signal all threads to stop.
        // Use a channel to queue all potential scores as another thread may already have sent a
        // different value.
        let (kind, units) = fight(input, boost);

        if kind == Kind::Immune {
            shared.done.store(true, Ordering::Relaxed);
            let _unused = shared.tx.send((boost, units));
        }
    }
}

fn fight(input: &Input, boost: i32) -> (Kind, i32) {
    let mut immune = input.immune.clone();
    let mut infection = input.infection.clone();
    let mut attacks = vec![None; immune.len() + infection.len()];

    // Boost reindeer's immmune system.
    immune.iter_mut().for_each(|group| group.damage += boost);

    for turn in 1.. {
        // Target selection phase.
        let mut target_selection = |attacker: &[Group], defender: &mut [Group], kind: Kind| {
            for (from, group) in attacker.iter().enumerate() {
                let target = (0..defender.len())
                    .filter(|&to| {
                        defender[to].chosen < turn && group.actual_damage(&defender[to]) > 0
                    })
                    .max_by_key(|&to| {
                        (
                            group.actual_damage(&defender[to]),
                            defender[to].effective_power(),
                            defender[to].initiative,
                        )
                    });

                if let Some(to) = target {
                    // Attacks happen in descending order of initiative.
                    let index = attacks.len() - group.initiative as usize;
                    defender[to].chosen = turn;
                    attacks[index] = Some((kind, from, to));
                }
            }
        };

        // Turn order is important.
        immune.sort_unstable_by_key(Group::target_selection_order);
        infection.sort_unstable_by_key(Group::target_selection_order);

        target_selection(&immune, &mut infection, Kind::Immune);
        target_selection(&infection, &mut immune, Kind::Infection);

        // Attacking phase.
        let mut killed = 0;

        for next in &mut attacks {
            if let Some((kind, from, to)) = *next {
                if kind == Kind::Immune {
                    killed += immune[from].attack(&mut infection[to]);
                } else {
                    killed += infection[from].attack(&mut immune[to]);
                }
                *next = None;
            }
        }

        // It's possible to deadlock if groups become too weak to do any more damage.
        if killed == 0 {
            return (Kind::Draw, 0);
        }

        // Check for winner.
        immune.retain(|group| group.units > 0);
        infection.retain(|group| group.units > 0);

        if immune.is_empty() {
            return (Kind::Infection, infection.iter().map(|group| group.units).sum());
        }
        if infection.is_empty() {
            return (Kind::Immune, immune.iter().map(|group| group.units).sum());
        }
    }

    unreachable!()
}

/// Parsing the input relatively cleanly is a challenge by itself.
fn parse_group<'a>(input: &'a str, mask: &mut impl FnMut(&'a str) -> u32) -> Vec<Group> {
    let delimiters = [' ', '(', ')', ',', ';'];
    input
        .lines()
        .skip(1)
        .map(|line| {
            let tokens: Vec<_> = line.split(delimiters).collect();

            let units = tokens[0].signed();
            let hit_points = tokens[4].signed();
            let damage = tokens[tokens.len() - 6].signed();
            let initiative = tokens[tokens.len() - 1].signed();
            let attack = mask(tokens[tokens.len() - 5]);
            let weak = parse_list(&tokens, "weak", mask);
            let immune = parse_list(&tokens, "immune", mask);
            let chosen = 0;

            Group { units, hit_points, damage, initiative, weak, immune, attack, chosen }
        })
        .collect()
}

/// There can be any amount of weaknesses or immunities.
fn parse_list<'a>(tokens: &[&'a str], start: &str, mask: &mut impl FnMut(&'a str) -> u32) -> u32 {
    let end = ["weak", "immune", "with"];
    let mut elements = 0;

    if let Some(index) = tokens.iter().position(|&t| t == start) {
        let mut index = index + 2;
        while !end.contains(&tokens[index]) {
            elements |= mask(tokens[index]);
            index += 1;
        }
    }

    elements
}
