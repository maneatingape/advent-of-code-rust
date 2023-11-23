//! Add a `chunk` method to [`Iterator`] that duplicates the functionality of the unstable
//! [`array_chunks`] method.
//!
//! Using Rust's const generics, concrete implementations are provided for sizes 2 to 8 to handle
//! the most common situations. Once [`array_chunks`] is stablized then this module can be removed.
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
        Chunk::<Self, N> { iter: self }
    }
}

impl<I: Iterator> Iterator for Chunk<I, 2> {
    type Item = [I::Item; 2];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        Some([a, b])
    }
}

impl<I: Iterator> Iterator for Chunk<I, 3> {
    type Item = [I::Item; 3];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        Some([a, b, c])
    }
}

impl<I: Iterator> Iterator for Chunk<I, 4> {
    type Item = [I::Item; 4];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        let d = self.iter.next()?;
        Some([a, b, c, d])
    }
}

impl<I: Iterator> Iterator for Chunk<I, 5> {
    type Item = [I::Item; 5];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        let d = self.iter.next()?;
        let e = self.iter.next()?;
        Some([a, b, c, d, e])
    }
}

impl<I: Iterator> Iterator for Chunk<I, 6> {
    type Item = [I::Item; 6];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        let d = self.iter.next()?;
        let e = self.iter.next()?;
        let f = self.iter.next()?;
        Some([a, b, c, d, e, f])
    }
}

impl<I: Iterator> Iterator for Chunk<I, 7> {
    type Item = [I::Item; 7];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        let d = self.iter.next()?;
        let e = self.iter.next()?;
        let f = self.iter.next()?;
        let g = self.iter.next()?;
        Some([a, b, c, d, e, f, g])
    }
}

impl<I: Iterator> Iterator for Chunk<I, 8> {
    type Item = [I::Item; 8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        let d = self.iter.next()?;
        let e = self.iter.next()?;
        let f = self.iter.next()?;
        let g = self.iter.next()?;
        let h = self.iter.next()?;
        Some([a, b, c, d, e, f, g, h])
    }
}
