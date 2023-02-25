//! Add a `chunk` method to [`Iterator`] that duplicates the functionality of the unstable
//! [`array_chunks`] method.
//!
//! Using Rust's const generics, concrete implementations are provided for sizes 2, 3 and 4 to handle
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

impl<I, T> Iterator for Chunk<I, 2>
where
    I: Iterator<Item = T>,
{
    type Item = [T; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        Some([a, b])
    }
}

impl<I, T> Iterator for Chunk<I, 3>
where
    I: Iterator<Item = T>,
{
    type Item = [T; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        Some([a, b, c])
    }
}

impl<I, T> Iterator for Chunk<I, 4>
where
    I: Iterator<Item = T>,
{
    type Item = [T; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        let d = self.iter.next()?;
        Some([a, b, c, d])
    }
}
