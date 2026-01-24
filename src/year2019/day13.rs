//! # Care Package
//!
//! Keeps track of the `x` position of both the ball and paddle then uses the [`signum`] function
//! to provide input to the joystick that tracks the ball.
//!
//! Just for fun this solution will play an animated game in the console if
//! "--features frivolity" is enabled.
//!
//! [`signum`]: i64::signum
use super::intcode::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> usize {
    let mut computer = Computer::new(input);
    let mut blocks = 0;

    while let [_, _, State::Output(t)] = [computer.run(), computer.run(), computer.run()] {
        if t == 2 {
            blocks += 1;
        }
    }

    blocks
}

pub fn part2(input: &[i64]) -> i64 {
    let mut modified = input.to_vec();
    modified[0] = 2;

    let mut computer = Computer::new(&modified);
    let mut tiles = Vec::with_capacity(1_000);
    let mut stride = 0;
    let mut score = 0;
    let mut blocks = score;
    let mut ball: i64 = 0;
    let mut paddle: i64 = 0;

    loop {
        let x = match computer.run() {
            State::Input => {
                // Always track the ball
                let delta = (ball - paddle).signum();
                computer.input(delta);
                continue;
            }
            State::Output(x) => x,
            State::Halted => unreachable!(),
        };
        let State::Output(y) = computer.run() else {
            unreachable!();
        };
        let State::Output(t) = computer.run() else {
            unreachable!();
        };

        if x < 0 {
            score = t;
            if blocks == 0 {
                break score;
            }
        } else {
            if x >= stride {
                stride = x + 1;
            }
            let index = (stride * y + x) as usize;
            if index >= tiles.len() {
                tiles.resize(index + 1, 0);
            }

            match t {
                0 if tiles[index] == 2 => blocks -= 1,
                2 if tiles[index] != 2 => blocks += 1,
                3 => paddle = x,
                4 => ball = x,
                _ => (),
            }

            tiles[index] = t;
        }

        // Non essential but hilarious. Enable feature then run program in a command line
        // console to observe an animated game of breakout.
        #[cfg(feature = "frivolity")]
        draw(&tiles, stride, score, blocks);
    }
}

#[cfg(feature = "frivolity")]
fn draw(tiles: &[i64], stride: i64, score: i64, blocks: i64) {
    use crate::util::ansi::*;
    use std::fmt::Write as _;
    use std::thread::sleep;
    use std::time::Duration;

    // Wait until the initial screen is complete
    let paddle = tiles.iter().rposition(|&t| t == 3).unwrap_or(tiles.len());
    if tiles[paddle..].iter().filter(|&&t| t == 1).count() < 3 {
        return;
    }

    let s = &mut String::new();
    let _ = writeln!(s, "{WHITE}{BOLD}Blocks: {blocks}\tScore: {score} {RESET}");
    let mut y = 0;

    while stride * y < tiles.len() as i64 {
        for x in 0..stride {
            let index = (stride * y + x) as usize;
            let _unused = match tiles[index] {
                0 => write!(s, " "),
                1 if y == 0 => write!(s, "{GREEN}_{RESET}"),
                1 => write!(s, "{GREEN}|{RESET}"),
                2 => write!(s, "{BLUE}#{RESET}"),
                3 => write!(s, "{WHITE}{BOLD}={RESET}"),
                4 => write!(s, "{YELLOW}{BOLD}o{RESET}"),
                _ => unreachable!(),
            };
        }
        s.push('\n');
        y += 1;
    }

    println!("{HOME}{CLEAR}{s}");
    sleep(Duration::from_millis(20));
}
