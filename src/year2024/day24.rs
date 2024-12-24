//! # Crossed Wires
use crate::util::hash::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u64 {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let mut names = FastMap::new();
    let mut cache = FastMap::new();
    let mut ops = FastMap::new();

    for line in prefix.lines() {
        let (key, value) = line.split_once(": ").unwrap();

        let size = names.len();
        let index = *names.entry(key).or_insert(size);

        cache.insert(index, value.unsigned::<u64>());
    }

    for line in suffix.lines() {
        let tokens: Vec<_> = line.split(' ').collect();
        let op = tokens[1];

        let size = names.len();
        let left = *names.entry(tokens[0]).or_insert(size);

        let size = names.len();
        let right = *names.entry(tokens[2]).or_insert(size);

        let size = names.len();
        let to = *names.entry(tokens[4]).or_insert(size);

        ops.insert(to, (left, op, right));
    }

    let mut result = 0;

    for i in (0..64).rev() {
        let key = format!("z{i:02}");
        if let Some(&key) = names.get(key.as_str()) {
            result = (result << 1) | helper(&mut cache, &mut ops, key);
        }
    }

    result
}

pub fn part2(_input: &str) -> String {
    // let (_, suffix) = input.split_once("\n\n").unwrap();
    // let mut wires = FastMap::new();

    // println!("digraph G {{");

    // for i in 0..46 {
    //     if i < 45 {
    //         let key = format!("x{i:02}");
    //         println!("  {} [pos=\"{},{}!\"]", key, i * 2, 5);
    //         let value = key.clone();
    //         wires.insert(key, vec![value]);

    //         let key = format!("y{i:02}");
    //         println!("  {} [pos=\"{},{}!\"]", key, i * 2 + 1, 5);
    //         let value = key.clone();
    //         wires.insert(key, vec![value]);
    //     }

    //     let key = format!("z{i:02}");
    //     println!("  {} [pos=\"{},{}!\"]", key, i * 2, 0);
    // }

    // println!();

    // for (name, line) in suffix.lines().enumerate() {
    //     let tokens: Vec<_> = line.split(' ').collect();
    //     let [_, _, _, _, to] = tokens[..] else { unreachable!() };
    //     wires.entry(String::from(to)).or_insert_with(Vec::new).push(format!("{name}"));
    // }

    // let mut second = FastMap::new();

    // for (name, line) in suffix.lines().enumerate() {
    //     let tokens: Vec<_> = line.split(' ').collect();
    //     let [left, op, right, _, to] = tokens[..] else { unreachable!() };

    //     let shape = match op {
    //         "AND" => "square",
    //         "OR" => "hexagon",
    //         "XOR" => "triangle",
    //         _ => unreachable!(),
    //     };

    //     if left.starts_with('x') || right.starts_with('x') {
    //         let i: usize = left.unsigned();
    //         if op == "AND" {
    //             println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 4);
    //             second.insert(to, i);
    //         }
    //         if op == "XOR" {
    //             println!("{} [pos=\"{},{}!\"]", name, i * 2, 4);
    //             second.insert(to, i);
    //         }
    //     }
    //     if to.starts_with('z') {
    //         let i: usize = to.unsigned();
    //         println!("{} [pos=\"{},{}!\"]", name, i * 2, 1);
    //     }

    //     println!("  {name} [shape={shape}]");
    //     for edge in &wires[&String::from(left)] {
    //         println!("  {edge} -> {name} [label=\"{left}\"]");
    //     }
    //     for edge in &wires[&String::from(right)] {
    //         println!("  {edge} -> {name} [label=\"{right}\"]");
    //     }
    // }

    // for (name, line) in suffix.lines().enumerate() {
    //     let tokens: Vec<_> = line.split(' ').collect();
    //     let [left, op, right, _, _] = tokens[..] else { unreachable!() };

    //     if op == "AND" {
    //         if let Some(i) = second.get(left) {
    //             println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 3);
    //         }
    //         if let Some(i) = second.get(right) {
    //             println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 3);
    //         }
    //     }
    //     if op == "OR" {
    //         if let Some(i) = second.get(left) {
    //             println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 2);
    //         }
    //         if let Some(i) = second.get(right) {
    //             println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 2);
    //         }
    //     }
    // }

    // for i in 0..46 {
    //     let key = format!("z{i:02}");
    //     for edge in &wires[&key] {
    //         println!("  {edge} -> {key}");
    //     }
    // }

    String::from("n/a")
}

fn helper(
    cache: &mut FastMap<usize, u64>,
    ops: &mut FastMap<usize, (usize, &str, usize)>,
    key: usize,
) -> u64 {
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let (left, op, right) = ops[&key];
    let left = helper(cache, ops, left);
    let right = helper(cache, ops, right);

    let value = match op {
        "AND" => left & right,
        "OR" => left | right,
        "XOR" => left ^ right,
        _ => unreachable!(),
    };

    cache.insert(key, value);
    value
}
