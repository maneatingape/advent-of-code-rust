//! Extracts and parses signed and unsigned integers from surrounding text and whitespace.
//!
//! A common pattern in AoC is to parse and return `123`, `456` and `789` from input resembling:
//!
//! ```none
//!   Lorem ipsum 123 dolor 456 sit 789 amet
//! ```
//!
//! This module provides two [`&str`] extension methods [`iter_signed`] and [`iter_unsigned`]. The
//! reason for the separate methods is that some AoC inputs contains the `-` character as a
//! delimeter and this would cause numbers to be incorrectly parsed as negative.
//!
//! [`iter_unsigned`]: ParseUnsigned::iter_unsigned
//! [`iter_signed`]: ParseSigned::iter_signed
use std::iter::{Filter, Map};
use std::str::{FromStr, Split};

/// Much shorter alias for the trait return type.
type Wrapper<'a, T> = Map<Filter<Split<'a, fn(char) -> bool>, fn(&&str) -> bool>, fn(&str) -> T>;

/// This convenience method does the same thing as `s.parse().unwrap()` but without the
/// extra `<F as FromStr>::Err` type bound so that we can have shorter type signatures.
pub fn from<T: FromStr>(s: &str) -> T {
    match s.parse() {
        Ok(t) => t,
        Err(_) => panic!("Unable to parse \"{s}\""),
    }
}

/// This trait allows us to keep type safety, restricting the possiblities to only
/// `u32`, `u64` and `usize`.
pub trait Unsigned: FromStr {}
impl Unsigned for u8 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for usize {}

/// Rust closures have an unique type that only the compiler knows and that us mere
/// mortals are not allowed to ascertain. Fortunately we can coerce the type to a known
/// function signature by using intermediate variables.
pub trait ParseUnsigned {
    fn iter_unsigned<T: Unsigned>(&self) -> Wrapper<'_, T>;
}

impl ParseUnsigned for &str {
    fn iter_unsigned<T: Unsigned>(&self) -> Wrapper<'_, T> {
        let not_numeric: fn(char) -> bool = |c| !c.is_ascii_digit();
        let not_empty: fn(&&str) -> bool = |s| !s.is_empty();
        self.split(not_numeric).filter(not_empty).map(from)
    }
}

/// This trait allows us to keep type safety, restricting the possiblities to only
/// `i32` and `i64`.
pub trait Signed: FromStr {}
impl Signed for i32 {}
impl Signed for i64 {}

/// Essentially the same as `ParseUnsigned` but also considers the `-` character as part
/// of a number.
pub trait ParseSigned {
    fn iter_signed<T: Signed>(&self) -> Wrapper<'_, T>;
}

impl ParseSigned for &str {
    fn iter_signed<T: Signed>(&self) -> Wrapper<'_, T> {
        let not_numeric: fn(char) -> bool = |c| !c.is_ascii_digit() && c != '-';
        let not_empty: fn(&&str) -> bool = |s| !s.is_empty();
        self.split(not_numeric).filter(not_empty).map(from)
    }
}
