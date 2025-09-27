//! Combines common [operators](https://doc.rust-lang.org/book/appendix-02-operators.html)
//! and constants `0`, `1` and `10` to enable generic methods on integer types.
use std::ops::*;

pub trait Integer<T>:
    Copy
    + From<u8>
    + PartialEq
    + PartialOrd
    + Add<Output = T>
    + BitAnd<Output = T>
    + BitOr<Output = T>
    + BitXor<Output = T>
    + Div<Output = T>
    + Mul<Output = T>
    + Rem<Output = T>
    + Shl<u32, Output = T>
    + Shr<u32, Output = T>
    + Sub<Output = T>
{
    const ZERO: T;
    const ONE: T;
    const TEN: T;

    fn trailing_zeros(self) -> u32;
}

pub trait Unsigned<T>: Integer<T> {}

pub trait Signed<T>: Integer<T> + Neg<Output = T> {}

macro_rules! integer {
    ($($t:ty)*) => ($(
        impl Integer<$t> for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TEN: $t = 10;

            #[inline]
            fn trailing_zeros(self) -> u32 {
                <$t>::trailing_zeros(self)
            }
        }
    )*)
}

macro_rules! empty_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name<$t> for $t {}
    )*)
}

integer!(u8 u16 u32 u64 u128 usize i16 i32 i64 i128);
empty_trait!(Unsigned for u8 u16 u32 u64 u128 usize);
empty_trait!(Signed for i16 i32 i64 i128);
