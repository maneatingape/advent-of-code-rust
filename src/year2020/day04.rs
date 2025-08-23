//! # Passport Processing
//!
//! [Regular expressions](https://en.wikipedia.org/wiki/Regular_expression) are a good fit for this
//! problem. However as the principles of this crate are to avoid external dependencies and
//! maximize speed we'll instead hand code validation functions for each of the
//! passport field criteria.
use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::RangeInclusive;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let mut passport = Vec::new();
    let mut part_one = 0;
    let mut part_two = 0;

    for block in input.split("\n\n") {
        parse_block(&mut passport, block);

        if passport.len() == 7 {
            part_one += 1;
            part_two += passport.iter().all(validate_field) as u32;
        }

        passport.clear();
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn parse_block<'a>(passport: &mut Vec<[&'a str; 2]>, block: &'a str) {
    for pair @ [key, _] in block.split([':', ' ', '\n']).chunk::<2>() {
        if key != "cid" {
            passport.push(pair);
        }
    }
}

fn validate_field(&[key, value]: &[&str; 2]) -> bool {
    match key {
        "byr" => validate_range(value, 1920..=2002),
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
    range.contains(&s.unsigned())
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
