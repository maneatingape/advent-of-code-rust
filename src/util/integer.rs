//! Combines common [operators](https://doc.rust-lang.org/book/appendix-02-operators.html)
//! and constants `0`, `1` and `10` to enable generic methods on integer types.
use std::ops::*;

pub trait Integer:
    Copy
    + From<u8>
    + PartialOrd
    + Add<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;

    fn lowest_one(self) -> Option<u32>;
    fn minmax(self, rhs: Self) -> (Self, Self);
}

pub trait Unsigned: Integer {}

pub trait Signed: Integer + Neg<Output = Self> {}

macro_rules! integer {
    ($($t:ty)*) => ($(
        impl Integer for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TEN: Self = 10;

            #[inline]
            fn lowest_one(self) -> Option<u32> {
                self.lowest_one()
            }

            #[inline]
            fn minmax(self, rhs: Self) -> (Self, Self) {
                if self < rhs { (self, rhs) } else { (rhs, self) }
            }
        }
    )*)
}

macro_rules! marker_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {}
    )*)
}

integer!(u8 u16 u32 u64 u128 usize i16 i32 i64 i128 isize);
marker_trait!(Unsigned for u8 u16 u32 u64 u128 usize);
marker_trait!(Signed for i16 i32 i64 i128 isize);
