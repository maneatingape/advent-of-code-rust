//! # Advent of Code solutions in Rust, tuned for speed.
//!
//! [![badge]][link]
//!
//! [badge]: https://img.shields.io/badge/github-blue?style=for-the-badge&logo=github&labelColor=grey
//! [link]: https://github.com/maneatingape/advent-of-code-rust

// Portable SIMD API is enabled by "simd" feature.
#![cfg_attr(feature = "simd", allow(unstable_features), feature(portable_simd))]

/// Enumerates all combinations of years and days, passing them to a callback macro.
/// Used by `benchmark.rs`, `lib.rs`, `main.rs` and `test.rs`.
#[macro_export]
macro_rules! solutions {
    ($callback:ident) => {
        $callback! {
            year2015 "Help Santa by solving puzzles to fix the weather machine's snow function."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2016 "Defeat the Easter Bunny to save Christmas."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2017 "A technical support callout from the Elves escalates rapidly."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2018 "Travel through time to restore the festive timeline."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2019 "Rescue Santa from deep space with a solar system voyage."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2020 "What could go wrong trying to enjoy a well-deserved vacation?"
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2021 "Retrieve the keys to Santa's sleigh with an underwater submarine adventure."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2022 "Assist the Elves on their annual jungle expedition."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2023 "Restore global snow production."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2024 "Locate the Chief Historian in time for the big Christmas sleigh launch."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
                day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24 day25,

            year2025 "Finish the North Pole decorations in time for Christmas."
                day01 day02 day03 day04 day05 day06 day07 day08 day09 day10 day11 day12
        }
    };
}

/// Declares a public module for each year documented with its description and containing a
/// sub-module for each day.
///
/// Two interesting things:
/// * Module declaration is possible via macro.
/// * Rust allows the day sub-module declarations directly without needing an intermediate `mod.rs`
///   in each year folder.
#[macro_export]
macro_rules! library {
    ($($year:ident $description:literal $($day:ident)*),*) => {
        $(pub mod $year {
            #![doc = concat!("# ", $description)]
            $(pub mod $day;)*
        })*
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi bitset grid hash heap intcode integer iter math md5 parse point thread
);

solutions!(library);
