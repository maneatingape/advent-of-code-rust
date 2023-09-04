//! # Care Package
//!
//! Keeps track of the `x` position of both the ball and paddle then uses the [`signum`] function
//! to provide input to the joystick that tracks the ball.
//!
//! Just for fun this solution will play an animated game in the console if the `_draw`
//! function is uncommented.
//!
//! [`signum`]: i64::signum
use super::day09::intcode::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> usize {
    let mut computer = Computer::new(input);
    let mut tiles = [0; 44 * 20];

    loop {
        let State::Output(x) = computer.run() else {
            break;
        };
        let State::Output(y) = computer.run() else {
            break;
        };
        let State::Output(t) = computer.run() else {
            break;
        };
        tiles[(44 * y + x) as usize] = t;
    }

    tiles.iter().filter(|&&t| t == 2).count()
}

pub fn part2(input: &[i64]) -> i64 {
    let mut modified = input.to_vec();
    modified[0] = 2;

    let mut computer = Computer::new(&modified);
    let mut tiles = [0; 44 * 20];
    let mut score = 0;
    let mut blocks = score;
    let mut ball: i64 = 0;
    let mut paddle: i64 = 0;

    loop {
        let x = match computer.run() {
            State::Input => {
                // Always track the ball
                let delta = (ball - paddle).signum();
                computer.input(&[delta]);
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
            let index = (44 * y + x) as usize;

            match t {
                0 if tiles[index] == 2 => blocks -= 1,
                2 if tiles[index] != 2 => blocks += 1,
                3 => paddle = x,
                4 => ball = x,
                _ => (),
            }

            tiles[index] = t;
        }

        // Non essential but hilarious. Uncomment `_draw` function then
        // run program on the command line to observe an animated game of breakout.
        //_draw(&tiles, score, blocks);
    }
}

fn _draw(tiles: &[i64], score: i64, blocks: i64) {
    use crate::util::ansi::*;
    use std::thread::sleep;
    use std::time::Duration;

    // Wait until the initial screen is complete
    if tiles[879] != 1 {
        return;
    }

    let mut s = String::new();
    s.push_str(&format!("{HOME}{CLEAR}{WHITE}{BOLD}Blocks: {blocks}\tScore: {score} {RESET}\n"));

    for y in 0..20 {
        for x in 0..44 {
            match tiles[44 * y + x] {
                0 => s.push(' '),
                1 => s.push_str(&format!("{GREEN}#{RESET}")),
                2 => s.push_str(&format!("{BLUE}o{RESET}")),
                3 => s.push_str(&format!("{WHITE}{BOLD}={RESET}")),
                4 => s.push_str(&format!("{YELLOW}{BOLD}.{RESET}")),
                _ => unreachable!(),
            }
        }
        s.push('\n');
    }

    println!("{s}");
    sleep(Duration::from_millis(20));
}
