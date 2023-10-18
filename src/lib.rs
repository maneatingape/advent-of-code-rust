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
    invalid_reference_casting,
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
#![warn(
    clippy::bool_to_int_with_if,
    clippy::borrow_as_ptr,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_precision_loss,
    clippy::cast_ptr_alignment,
    clippy::checked_conversions,
    clippy::cloned_instead_of_copied,
    clippy::copy_iterator,
    clippy::default_trait_access,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_not_else,
    clippy::implicit_clone,
    clippy::inconsistent_struct_constructor,
    clippy::index_refutable_slice,
    clippy::inefficient_to_string,
    clippy::inline_always,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_statements,
    clippy::iter_not_returning_iterator,
    clippy::large_digit_groups,
    clippy::large_futures,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::manual_assert,
    clippy::manual_instant_elapsed,
    clippy::manual_let_else,
    clippy::manual_ok_or,
    clippy::manual_string_new,
    clippy::map_unwrap_or,
    clippy::match_bool,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::maybe_infinite_iter,
    clippy::mismatching_type_param_order,
    clippy::missing_errors_doc,
    clippy::missing_fields_in_debug,
    clippy::mut_mut,
    clippy::needless_bitwise_bool,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::no_effect_underscore_binding,
    clippy::no_mangle_with_rust_abi,
    clippy::option_option,
    clippy::ptr_as_ptr,
    clippy::ptr_cast_constness,
    clippy::range_minus_one,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::ref_binding_to_reference,
    clippy::ref_option_ref,
    clippy::return_self_not_must_use,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::stable_sort_primitive,
    clippy::string_add_assign,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::transmute_ptr_to_ptr,
    clippy::trivially_copy_pass_by_ref,
    clippy::unchecked_duration_subtraction,
    clippy::unicode_not_nfc,
    clippy::uninlined_format_args,
    clippy::unnecessary_box_returns,
    clippy::unnecessary_join,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns,
    clippy::unsafe_derive_deserialize,
    clippy::unused_async,
    clippy::unused_self,
    clippy::used_underscore_binding,
    clippy::verbose_bit_mask,
    clippy::zero_sized_map_values
)]

//! <!-- Cherry pick some stylistic Clippy restriction lints -->
#![warn(
    clippy::empty_structs_with_brackets,
    clippy::float_arithmetic,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::needless_raw_strings,
    clippy::redundant_type_annotations,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix
)]

/// # Utility modules to handle common recurring Advent of Code patterns.
pub mod util {
    pub mod ansi;
    pub mod grid;
    pub mod hash;
    pub mod heap;
    pub mod integer;
    pub mod iter;
    pub mod math;
    pub mod md5;
    pub mod parse;
    pub mod point;
    pub mod slice;
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

/// # Defeat the Easter Bunny to save Christmas.
pub mod year2016 {
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
}

/// # Rescue Santa from deep space with a solar system adventure.
pub mod year2019 {
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
    pub mod intcode;
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
