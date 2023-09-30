//! # Corporate Policy
//!
//! Like the [previous day] we rely on the special structure of the input to solve
//! more efficiently than the general case.
//!
//! The key observation is that if there are no straights or pairs in the first 4 digits then the
//! next lowest valid sequence will end in a string of the form `aabcc`,
//! compressing 2 pairs and a straight into 5 digits.
//!
//! * If the current string ends with `xxyzz` then increment the third digit and wrap around
//!   to `aabcc`.
//! * The 5 digit sequence cannot start with any letter from `g` to `o` inclusive or it would
//!   contain an invalid character somewhere in the sequence.
//!
//! [previous day]: crate::year2015::day10
use std::str::from_utf8;

type Password = [u8; 8];
type Input = [Password; 2];

pub fn parse(input: &str) -> Input {
    let password = clean(input.trim().as_bytes().try_into().unwrap());

    // No pairs in the first 4 characters
    let pair = |i, j| password[i] == password[j];
    assert!(!(pair(0, 1) | pair(1, 2) | pair(2, 3)));

    // No straights in the first 4 characters
    let sequence = |i, j| password[j] > password[i] && password[j] - password[i] == 1;
    assert!(!(sequence(1, 2) && (sequence(0, 1) || sequence(2, 3))));

    // No potential carry in the third character
    assert_ne!(password[2], b'z');

    let first = next_password(password);
    let second = next_password(first);
    [first, second]
}

pub fn part1(input: &Input) -> &str {
    from_utf8(&input[0]).unwrap()
}

pub fn part2(input: &Input) -> &str {
    from_utf8(&input[1]).unwrap()
}

/// Sanitize the input to make sure it has no invalid characters. We increment the first invalid
/// character found, for example `abcixyz` becomes `abcjaaa`.
fn clean(mut password: Password) -> Password {
    let mut reset = false;

    for digit in &mut password {
        if reset {
            *digit = b'a';
        } else if matches!(digit, b'i' | b'o' | b'l') {
            *digit += 1;
            reset = true;
        }
    }

    password
}

/// Find the next valid 5 digit sequence of form `aabcc`.
fn next_password(mut password: Password) -> Password {
    // If the sequence would contain any illegal character, then skip to the next possible
    // valid sequence starting with `p`.
    if (b'g'..b'o').contains(&password[3]) {
        return fill(password, b'p');
    }

    // If there's room then check if a sequence starting with the current character is
    // higher than the current password.
    if password[3] <= b'x' {
        let candidate = fill(password, password[3]);
        if candidate > password {
            return candidate;
        }
    }

    // Otherwise we need to increment the first digit of the sequence.
    if password[3] == b'x' {
        // If it starts with `x` then increment the third digit and wrap around.
        password[2] += if matches!(password[2], b'h' | b'n' | b'k') { 2 } else { 1 };
        fill(password, b'a')
    } else if password[3] == b'f' {
        // If it would enter the invalid range from `g` to `o` then take the next valid start `p`.
        fill(password, b'p')
    } else {
        // Otherwise increment the first digit.
        fill(password, password[3] + 1)
    }
}

/// Creates a sequence of form `aabcc` from an arbitrary starting character.
fn fill(mut password: Password, start: u8) -> Password {
    password[3] = start;
    password[4] = start;
    password[5] = start + 1;
    password[6] = start + 2;
    password[7] = start + 2;
    password
}
