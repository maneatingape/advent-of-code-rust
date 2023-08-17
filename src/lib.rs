//! # Advent of Code solutions in Rust, tuned for speed.
//! [![github]](https://github.com/maneatingape/advent-of-code-rust)
//!
//! [github]: https://img.shields.io/badge/github-blue?style=for-the-badge&logo=github&labelColor=grey

//! <!-- Configure rustdoc -->
#![doc(html_logo_url = "https://maneatingape.github.io/advent-of-code-rust/logo.png")]
#![allow(rustdoc::private_intra_doc_links)]

//! <!-- Stricter Rustc lints -->
#![warn(
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    //invalid_reference_casting, // Not supported by GitHub actions yet
    keyword_idents,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences
)]

//! <!-- Clippy Pedantic lints excluding some noisy rules -->
#![warn(clippy::pedantic)]
#![allow(
    clippy::similar_names,
    clippy::many_single_char_names,
    clippy::module_name_repetitions,
    clippy::unreadable_literal,
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::enum_glob_use,
    clippy::wildcard_imports,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::missing_panics_doc,
    clippy::match_on_vec_items,
    clippy::range_plus_one,
    clippy::naive_bytecount,
    clippy::implicit_hasher
)]

//! <!-- Cherry pick some stylistic Clippy restriction lints -->
#![warn(
    clippy::empty_structs_with_brackets,
    clippy::float_arithmetic,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    // clippy::redundant_type_annotations)] // Not supported by GitHub actions yet
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix
)]

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

/// # Rescue Santa from deep space with a solar system adventure.
pub mod year2019 {
    pub mod day01;
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
