use std::iter::{Filter, Map};
use std::str::{FromStr, Split};

type Wrapper<'a, T> = Map<Filter<Split<'a, fn(char) -> bool>, fn(&&str) -> bool>, fn(&str) -> T>;

pub fn from<T: FromStr>(s: &str) -> T {
    match s.parse() {
        Ok(t) => t,
        Err(_) => panic!("Unable to parse \"{s}\""),
    }
}

pub trait Unsigned: FromStr {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for usize {}

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

pub trait Signed: FromStr {}
impl Signed for i32 {}
impl Signed for i64 {}

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
