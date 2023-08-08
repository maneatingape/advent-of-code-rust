//! # Rust solutions to the annual Advent of Code challenge tuned for speed.
//!
//! [GitHub Repo](https://github.com/maneatingape/advent-of-code-rust)

// Configure rustdoc
#![doc(html_logo_url = "https://maneatingape.github.io/advent-of-code-rust/logo.png")]
#![allow(rustdoc::private_intra_doc_links)]
// Include Clippy pedantic lints excluding noisy rules.
#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_on_vec_items)]
#![allow(clippy::range_plus_one)]
#![allow(clippy::naive_bytecount)]
#![allow(clippy::implicit_hasher)]

/// # Utility modules to handle common recurring Advent of Code patterns.
pub mod util {
    pub mod grid;
    pub mod hash;
    pub mod iter;
    pub mod math;
    pub mod md5;
    pub mod parse;
    pub mod point;
    pub mod slice;
}

/// # Assist the Elves on their annual jungle expedition.
pub mod year2022 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

/// # Retrieve the keys to Santa's sleigh with an underwater submarine adventure.
pub mod year2021 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

/// # What could go wrong trying to enjoy a well deserved vacation?
pub mod year2020 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

/// # Help Santa by solving puzzles to fix the weather machine's snow function.
pub mod year2015 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
}
