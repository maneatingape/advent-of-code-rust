//! Extracts and parses signed and unsigned integers from surrounding text and whitespace.
//!
//! A common pattern in Advent of Code is to parse and return `123`, `456` and `789` from input
//! resembling:
//!
//! ```none
//!   Lorem ipsum 123 dolor 456 sit 789 amet
//! ```
//!
//! This module provides two [`&str`] extension methods [`iter_signed`] and [`iter_unsigned`]. The
//! reason for the separate methods is that some Advent of Code inputs contains the `-` character
//! as a delimeter and this would cause numbers to be incorrectly parsed as negative.
//!
//! [`iter_unsigned`]: ParseOps::iter_unsigned
//! [`iter_signed`]: ParseOps::iter_signed
use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg};
use std::str::Bytes;

pub trait ParseByte {
    fn to_decimal(self) -> u8;
}

impl ParseByte for u8 {
    #[inline]
    fn to_decimal(self) -> u8 {
        self.wrapping_sub(b'0')
    }
}

// Traits allow us to keep type safety, restricting the possiblities to only integer types.
pub trait Unsigned: From<u8> + Add<Output = Self> + Mul<Output = Self> {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for usize {}

pub trait Signed: From<u8> + Add<Output = Self> + Mul<Output = Self> + Neg<Output = Self> {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}

pub struct ParseUnsigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<T>,
}

pub struct ParseSigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<T>,
}

pub trait ParseOps {
    fn unsigned<T: Unsigned>(&self) -> T;
    fn signed<T: Signed>(&self) -> T;
    fn iter_unsigned<T: Unsigned>(&self) -> ParseUnsigned<'_, T>;
    fn iter_signed<T: Signed>(&self) -> ParseSigned<'_, T>;
}

impl ParseOps for &str {
    fn unsigned<T: Unsigned>(&self) -> T {
        match try_unsigned(&mut self.bytes()) {
            Some(t) => t,
            None => panic!("Unable to parse \"{self}\""),
        }
    }

    fn signed<T: Signed>(&self) -> T {
        match try_signed(&mut self.bytes()) {
            Some(t) => t,
            None => panic!("Unable to parse \"{self}\""),
        }
    }

    fn iter_unsigned<T: Unsigned>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.bytes(), phantom: PhantomData }
    }

    fn iter_signed<T: Signed>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.bytes(), phantom: PhantomData }
    }
}

impl<T: Unsigned> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        (lower / 3, upper.map(|u| u / 3))
    }

    fn next(&mut self) -> Option<Self::Item> {
        try_unsigned(&mut self.bytes)
    }
}

impl<T: Signed> Iterator for ParseSigned<'_, T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        (lower / 3, upper.map(|u| u / 3))
    }

    fn next(&mut self) -> Option<Self::Item> {
        try_signed(&mut self.bytes)
    }
}

fn try_unsigned<T: Unsigned>(bytes: &mut Bytes) -> Option<T> {
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = byte.to_decimal();

        if digit < 10 {
            break T::from(digit);
        }
    };

    loop {
        let Some(byte) = bytes.next() else { break Some(n) };
        let digit = byte.to_decimal();

        if digit < 10 {
            n = T::from(10) * n + T::from(digit);
        } else {
            break Some(n);
        }
    }
}

fn try_signed<T: Signed>(bytes: &mut Bytes) -> Option<T> {
    let (mut n, negative) = loop {
        let byte = bytes.next()?;
        let digit = byte.to_decimal();

        if digit == 253 {
            break (T::from(0), true);
        }
        if digit < 10 {
            break (T::from(digit), false);
        }
    };

    loop {
        let Some(byte) = bytes.next() else {
            break Some(if negative { -n } else { n });
        };
        let digit = byte.to_decimal();

        if digit < 10 {
            n = T::from(10) * n + T::from(digit);
        } else {
            break Some(if negative { -n } else { n });
        }
    }
}
