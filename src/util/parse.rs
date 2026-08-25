//! Extracts and parses signed and unsigned integers from surrounding text and whitespace.
//!
//! A common pattern in Advent of Code is to parse and return `123`, `456` and `789` from input
//! resembling the following form:
//!
//! ```none
//! Lorem ipsum 123 dolor 456 sit 789 amet
//! ```
//!
//! This module provides two [`&str`] extension methods [`iter_signed`] and [`iter_unsigned`]. The
//! reason for the separate methods is that some Advent of Code inputs contain the `-` character
//! as a delimiter and this would cause numbers to be incorrectly parsed as negative.
//!
//! [`iter_unsigned`]: ParseOps::iter_unsigned
//! [`iter_signed`]: ParseOps::iter_signed
use std::marker::PhantomData;

use crate::util::integer::*;

const MINUS: u8 = b'-'.wrapping_sub(b'0');

pub trait ParseByte {
    fn to_decimal<T: Integer>(self) -> T;
}

impl ParseByte for u8 {
    #[inline]
    fn to_decimal<T: Integer>(self) -> T {
        T::from(self.wrapping_sub(b'0'))
    }
}

pub trait ParseOps {
    fn unsigned<T: Unsigned>(&self) -> T;
    fn signed<T: Signed>(&self) -> T;
    fn iter_unsigned<T: Unsigned>(&self) -> impl Iterator<Item = T>;
    fn iter_signed<T: Signed>(&self) -> impl Iterator<Item = T>;
}

impl<S: AsRef<str> + ?Sized> ParseOps for S {
    #[inline]
    fn unsigned<T: Unsigned>(&self) -> T {
        let mut bytes = self.as_ref().bytes();
        try_unsigned(&mut bytes).unwrap()
    }

    #[inline]
    fn signed<T: Signed>(&self) -> T {
        let mut bytes = self.as_ref().bytes();
        try_signed(&mut bytes).unwrap()
    }

    #[inline]
    fn iter_unsigned<T: Unsigned>(&self) -> impl Iterator<Item = T> {
        let bytes = self.as_ref().bytes();
        ParseUnsigned { bytes, phantom: PhantomData }
    }

    #[inline]
    fn iter_signed<T: Signed>(&self) -> impl Iterator<Item = T> {
        let bytes = self.as_ref().bytes();
        ParseSigned { bytes, phantom: PhantomData }
    }
}

struct ParseUnsigned<I, T> {
    bytes: I,
    phantom: PhantomData<T>,
}

impl<I: Iterator<Item = u8>, T: Unsigned> Iterator for ParseUnsigned<I, T> {
    type Item = T;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        (lower / 3, upper.map(|u| u / 3))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        try_unsigned(&mut self.bytes)
    }
}

struct ParseSigned<I, T> {
    bytes: I,
    phantom: PhantomData<T>,
}

impl<I: Iterator<Item = u8>, T: Signed> Iterator for ParseSigned<I, T> {
    type Item = T;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        (lower / 3, upper.map(|u| u / 3))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        try_signed(&mut self.bytes)
    }
}

fn try_unsigned<T: Unsigned>(bytes: &mut impl Iterator<Item = u8>) -> Option<T> {
    let mut n = loop {
        let digit = bytes.next()?.to_decimal();
        if digit < 10 {
            break T::from(digit);
        }
    };

    for byte in bytes {
        let digit = byte.to_decimal();
        if digit >= 10 {
            break;
        }
        n = T::TEN * n + T::from(digit);
    }

    Some(n)
}

fn try_signed<T: Signed>(bytes: &mut impl Iterator<Item = u8>) -> Option<T> {
    let (mut n, negative) = loop {
        let digit = bytes.next()?.to_decimal();
        if digit == MINUS {
            break (T::ZERO, true);
        }
        if digit < 10 {
            break (T::from(digit), false);
        }
    };

    for byte in bytes {
        let digit = byte.to_decimal();
        if digit >= 10 {
            break;
        }
        n = T::TEN * n + T::from(digit);
    }

    Some(if negative { -n } else { n })
}
