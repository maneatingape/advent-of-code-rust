//! # Reindeer Olympics
//!
//! In order to make things easier we create a function to calculate the distance travelled by a
//! reindeer at any arbitrary time.
use crate::util::iter::*;
use crate::util::parse::*;

type Reindeer = [u32; 3];

pub fn parse(input: &str) -> Vec<Reindeer> {
    input.iter_unsigned().chunk::<3>().collect()
}

pub fn part1(input: &[Reindeer]) -> u32 {
    part1_testable(input, 2503)
}

pub fn part2(input: &[Reindeer]) -> u32 {
    part2_testable(input, 2503)
}

pub fn part1_testable(input: &[Reindeer], time: u32) -> u32 {
    input.iter().map(|&r| distance(r, time)).max().unwrap()
}

pub fn part2_testable(input: &[Reindeer], time: u32) -> u32 {
    let mut score = vec![0; input.len()];
    let mut distances = vec![0; input.len()];

    for minute in 1..time {
        let mut lead = 0;

        for (index, &reindeer) in input.iter().enumerate() {
            let next = distance(reindeer, minute);
            distances[index] = next;
            lead = lead.max(next);
        }

        for (index, &distance) in distances.iter().enumerate() {
            if distance == lead {
                score[index] += 1;
            }
        }
    }

    *score.iter().max().unwrap()
}

fn distance([speed, fly, rest]: Reindeer, time: u32) -> u32 {
    let total = fly + rest;
    let complete = time / total;
    let partial = (time % total).min(fly);

    speed * (fly * complete + partial)
}
