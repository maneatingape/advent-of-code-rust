use std::ops::{Add, Mul, Neg};

pub trait Unsigned<T>: From<u8> + Add<Output = T> + Mul<Output = T> {
    const ZERO: T;
    const ONE: T;
    const TEN: T;
}

macro_rules! unsigned {
    ($($t:ty)*) => ($(
        impl Unsigned<$t> for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TEN: $t = 10;
        }
    )*)
}

unsigned!(u8 u16 u32 u64 usize);

pub trait Signed<T>: From<u8> + Add<Output = T> + Mul<Output = T> + Neg<Output = T> {
    const MINUS_ONE: T;
    const ZERO: T;
    const ONE: T;
    const TEN: T;
}

macro_rules! signed {
    ($($t:ty)*) => ($(
        impl Signed<$t> for $t {
            const MINUS_ONE: $t = -1;
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TEN: $t = 10;
        }
    )*)
}

signed!(i16 i32 i64);
