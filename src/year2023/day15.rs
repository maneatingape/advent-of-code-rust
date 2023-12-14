use crate::util::parse::*;

type Input = (usize, usize);

struct Item<'a> {
    label: &'a [u8],
    lens: usize,
}

pub fn parse(input: &str) -> Input {
    let mut part_one = 0;
    let mut part_two = 0;
    let mut boxes: Vec<Vec<Item<'_>>> = (0..256).map(|_| Vec::new()).collect();

    for step in input.trim().as_bytes().split(|&b| b == b',') {
        let size = step.len();
        part_one += hash(step);

        if step[size - 1] == b'-' {
            let label = &step[..size - 1];
            let hash = hash(label);
            let slot = &mut boxes[hash];

            if let Some(i) = slot.iter().position(|item| item.label == label) {
                slot.remove(i);
            }
        } else {
            let label = &step[..size - 2];
            let hash = hash(label);
            let slot = &mut boxes[hash];
            let lens = step[size - 1].to_decimal() as usize;

            if let Some(i) = slot.iter().position(|item| item.label == label) {
                slot[i].lens = lens;
            } else {
                slot.push(Item { label, lens });
            }
        }
    }

    for (i, next) in boxes.iter().enumerate() {
        for (j, item) in next.iter().enumerate() {
            part_two += (i + 1) * (j + 1) * item.lens;
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

#[inline]
fn hash(slice: &[u8]) -> usize {
    slice.iter().fold(0, |acc, &b| ((acc + b as usize) * 17) & 0xff)
}
