//! # Cryostasis
//!
//! Plays the game automatically, solving the weight puzzle using
//! [Gray codes](https://en.wikipedia.org/wiki/Gray_code) to check every combination of items
//! by swapping only one item at a time.
//!
//! Makes some assumptions:
//! * The ship's layout contains no loops, so a depth first search will explore every room
//!   then return to the starting point.
//! * The 5 dangerous items are common across all inputs.
//! * No items are called "north", "south", "west" or "east".
//! * The final room is called "Pressure-Sensitive Floor".
//!
//! If these assumptions hold then this solution will solve any arbitrary combination of ship
//! layout and items.
//!
//! Just for fun this solution can be played interactively on the command line if
//! "--features frivolity" is enabled.
use super::intcode::*;
use crate::util::hash::*;
use crate::util::parse::*;
use std::fmt::Write as _;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> String {
    if cfg!(feature = "frivolity") { play_manually(input) } else { play_automatically(input) }
}

pub fn part2(_input: &[i64]) -> &'static str {
    "n/a"
}

// Let a human play the game interactively.
fn play_manually(input: &[i64]) -> String {
    use std::io::stdin;

    let mut computer = Computer::new(input);
    let mut output = String::new();
    let mut input = String::new();

    loop {
        match computer.run() {
            State::Output(value) => {
                let ascii = (value as u8) as char;
                output.push(ascii);
            }
            State::Input => {
                pretty_print(&output);
                output.clear();
                let _unused = stdin().read_line(&mut input);
                computer.input_ascii(&input);
                input.clear();
            }
            State::Halted => {
                pretty_print(&output);
                output.retain(|c| c.is_ascii_digit());
                break output;
            }
        }
    }
}

// Use ANSI codes to colorize the output to highlight the text.
fn pretty_print(output: &str) {
    use crate::util::ansi::*;

    let mut buffer = String::new();
    let mut item = GREEN;

    for line in output.lines() {
        if line.starts_with('=') {
            let _ = write!(&mut buffer, "{BOLD}{WHITE}{line}{RESET}");
        } else if line.starts_with('-') {
            let _ = write!(&mut buffer, "{item}{line}{RESET}");
        } else if line.starts_with("Items here:") {
            item = YELLOW;
            buffer.push_str(line);
        } else {
            buffer.push_str(line);
        }
        buffer.push('\n');
    }

    println!("{buffer}");
}

fn play_automatically(input: &[i64]) -> String {
    let mut computer = Computer::new(input);
    let mut stack = Vec::new();
    let mut path = Vec::new();
    let mut inventory = Vec::new();

    // DFS through the ship, picking up all 8 safe items, then return to the starting point.
    explore(&mut computer, &mut stack, &mut path, &mut inventory);

    // Retrace our path back to the Security Checkpoint.
    let last = path.pop().unwrap();

    for direction in path {
        movement_silent(&mut computer, &direction);
    }

    // Use Gray codes to take or drop one item at a time, until we are exactly the right weight.
    // As an optimization we keep track of combinations of items that are too heavy or too light.
    // If we are adding an item to a collection that is already too heavy or vice-versa,
    // then we can skip the pressure plate check.
    let combinations: u32 = 1 << inventory.len();
    let mut output = String::new();
    let mut too_light = FastSet::new();
    let mut too_heavy = FastSet::new();

    for i in 1..combinations {
        let current = gray_code(i);
        let previous = gray_code(i - 1);
        let changed = current ^ previous;
        let index = changed.trailing_zeros() as usize;

        // Since we start with all items in our possesion, the meaning of bits in the gray code is
        // reversed. 0 is take an item and 1 is drop an item.
        if current & changed == 0 {
            take_item(&mut computer, &inventory[index]);

            if too_heavy.contains(&previous) {
                too_heavy.insert(current);
                continue;
            }
        } else {
            drop_item(&mut computer, &inventory[index]);

            if too_light.contains(&previous) {
                too_light.insert(current);
                continue;
            }
        };

        if matches!(movement_noisy(&mut computer, &last, &mut output), State::Halted) {
            // Keep only the password digits from Santa's response.
            output.retain(|b| b.is_ascii_digit());
            break;
        } else if output.contains("heavier") {
            too_light.insert(current);
        } else {
            too_heavy.insert(current);
        }

        output.clear();
    }

    output
}

fn explore(
    computer: &mut Computer,
    stack: &mut Vec<String>,
    path: &mut Vec<String>,
    inventory: &mut Vec<String>,
) {
    let direction = stack.last().map_or("none", |d| d.as_str());
    let reverse = opposite(direction);

    let mut output = String::new();
    movement_noisy(computer, direction, &mut output);

    for line in output.lines() {
        if line.starts_with("== Pressure-Sensitive Floor ==") {
            path.clone_from(stack);
            return;
        } else if let Some(suffix) = line.strip_prefix("- ") {
            if opposite(suffix) == "none" {
                let item = String::from(suffix);
                if !dangerous(&item) {
                    take_item(computer, &item);
                    inventory.push(item);
                }
            } else {
                let direction = String::from(suffix);
                if direction != reverse {
                    stack.push(direction);
                    explore(computer, stack, path, inventory);
                    stack.pop();
                }
            }
        }
    }

    movement_silent(computer, reverse);
}

fn opposite(direction: &str) -> &'static str {
    match direction {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => "none",
    }
}

fn dangerous(item: &str) -> bool {
    matches!(
        item,
        "escape pod" | "giant electromagnet" | "infinite loop" | "molten lava" | "photons"
    )
}

fn movement_noisy(computer: &mut Computer, direction: &str, output: &mut String) -> State {
    if direction != "none" {
        computer.input_ascii(&format!("{direction}\n"));
    }
    loop {
        match computer.run() {
            State::Output(value) => {
                let ascii = (value as u8) as char;
                output.push(ascii);
            }
            other => break other,
        }
    }
}

fn movement_silent(computer: &mut Computer, direction: &str) {
    if direction != "none" {
        computer.input_ascii(&format!("{direction}\n"));
        drain_output(computer);
    }
}

fn take_item(computer: &mut Computer, item: &str) {
    computer.input_ascii(&format!("take {item}\n"));
    drain_output(computer);
}

fn drop_item(computer: &mut Computer, item: &str) {
    computer.input_ascii(&format!("drop {item}\n"));
    drain_output(computer);
}

// A quirk of the intcode program is that commands can't be stacked. We must first read all the
// input from previous command before the next command can be submitted.
fn drain_output(computer: &mut Computer) {
    while let State::Output(_) = computer.run() {}
}

// Convert an normal binary number to its Gray Code equivalent
fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}
