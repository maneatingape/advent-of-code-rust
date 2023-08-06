//! # Passport Processing
//!
//! [Regular expressions](https://en.wikipedia.org/wiki/Regular_expression) are a good fit for this
//! problem. However as the principles of this crate are to avoid external dependencies and
//! maximize speed we'll instead hand code validation functions for each of the
//! passport field criteria.
use crate::util::iter::*;
use std::ops::RangeInclusive;

type Passport<'a> = Vec<[&'a str; 2]>;

pub fn parse(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(parse_block).collect()
}

pub fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|passport| passport.len() == 7).count()
}

pub fn part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|passport| passport.len() == 7)
        .filter(|passport| passport.iter().all(validate_field))
        .count()
}

fn parse_block(block: &str) -> Passport {
    let mut fields = Vec::with_capacity(7);

    for pair @ [key, _] in block.split([':', ' ', '\n']).chunk::<2>() {
        if key != "cid" {
            fields.push(pair);
        }
    }

    fields
}

fn validate_field(&[key, value]: &[&str; 2]) -> bool {
    match key {
        "byr" => validate_range(value, 1920..=2022),
        "iyr" => validate_range(value, 2010..=2020),
        "eyr" => validate_range(value, 2020..=2030),
        "hgt" => validate_height(value),
        "hcl" => validate_hair_color(value),
        "ecl" => validate_eye_color(value),
        "pid" => validate_passport_id(value),
        _ => unreachable!(),
    }
}

fn validate_range(s: &str, range: RangeInclusive<u32>) -> bool {
    s.parse().is_ok_and(|n| range.contains(&n))
}

fn validate_height(hgt: &str) -> bool {
    if hgt.len() == 4 && hgt.ends_with("in") {
        validate_range(&hgt[..2], 59..=76)
    } else if hgt.len() == 5 && hgt.ends_with("cm") {
        validate_range(&hgt[..3], 150..=193)
    } else {
        false
    }
}

fn validate_hair_color(hcl: &str) -> bool {
    let hcl = hcl.as_bytes();
    hcl.len() == 7 && hcl[0] == b'#' && hcl[1..].iter().all(u8::is_ascii_hexdigit)
}

fn validate_eye_color(ecl: &str) -> bool {
    matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn validate_passport_id(pid: &str) -> bool {
    pid.len() == 9 && pid.bytes().all(|b| b.is_ascii_digit())
}
