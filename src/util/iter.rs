//! Add a `chunk` method to [`Iterator`] that duplicates the functionality of the unstable
//! [`array_chunks`] method.
//!
//! Using Rust's const generics, concrete implementations are provided for sizes 2 to 12 to handle
//! the most common situations. Once [`array_chunks`] is stabilized then this module can be removed.
//!
//! [`array_chunks`]: std::iter::Iterator::array_chunks
pub struct Chunk<I: Iterator, const N: usize> {
    iter: I,
}

pub trait ChunkOps: Iterator + Sized {
    fn chunk<const N: usize>(self) -> Chunk<Self, N>;
}

impl<I: Iterator> ChunkOps for I {
    fn chunk<const N: usize>(self) -> Chunk<Self, N> {
        Chunk { iter: self }
    }
}

macro_rules! iterator {
    ($n:literal, $($var:ident),+) => {
        impl<I: Iterator> Iterator for Chunk<I, $n> {
            type Item = [I::Item; $n];

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                Some([$({
                    let $var = self.iter.next()?;
                    $var
                }),+])
            }
        }
    };
}

iterator!(2, a, b);
iterator!(3, a, b, c);
iterator!(4, a, b, c, d);
iterator!(5, a, b, c, d, e);
iterator!(6, a, b, c, d, e, f);
iterator!(7, a, b, c, d, e, f, g);
iterator!(8, a, b, c, d, e, f, g, h);
iterator!(9, a, b, c, d, e, f, g, h, i);
iterator!(10, a, b, c, d, e, f, g, h, i, j);
iterator!(11, a, b, c, d, e, f, g, h, i, j, k);
iterator!(12, a, b, c, d, e, f, g, h, i, j, k, l);
