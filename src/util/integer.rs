use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, BitAnd, Div, Mul, Neg, Rem, Shr, Sub};

pub trait Integer<T>:
    Copy
    + From<u8>
    + PartialEq
    + PartialOrd
    + Add<Output = T>
    + BitAnd<Output = T>
    + Div<Output = T>
    + Mul<Output = T>
    + Rem<Output = T>
    + Shr<Output = T>
    + Sub<Output = T>
{
    const ZERO: T;
    const ONE: T;
    const TEN: T;
}

macro_rules! Integer {
    ($($t:ty)*) => ($(
        impl Integer<$t> for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TEN: $t = 10;
        }

    )*)
}

Integer!(u8 u16 u32 u64 usize i16 i32 i64);

pub trait Unsigned<T>: Integer<T> {}

macro_rules! unsigned {
    ($($t:ty)*) => ($(
        impl Unsigned<$t> for $t {}
    )*)
}

unsigned!(u8 u16 u32 u64 usize);

pub trait Signed<T>: Integer<T> + Neg<Output = T> {}

macro_rules! signed {
    ($($t:ty)*) => ($(
        impl Signed<$t> for $t {}
    )*)
}

signed!(i16 i32 i64);
