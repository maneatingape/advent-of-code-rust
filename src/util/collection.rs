pub struct Chunked<I, const N: usize>(I);

pub trait ChunkedOps
where
    Self: Sized,
{
    fn chunked<const N: usize>(self) -> Chunked<Self, N>;
}

impl<I> ChunkedOps for I
where
    I: Iterator,
{
    fn chunked<const N: usize>(self) -> Chunked<Self, N> {
        Chunked::<Self, N>(self)
    }
}

impl<I, T> Iterator for Chunked<I, 2>
where
    I: Iterator<Item = T>,
{
    type Item = [T; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.0.next()?;
        Some([a, b])
    }
}

impl<I, T> Iterator for Chunked<I, 3>
where
    I: Iterator<Item = T>,
{
    type Item = [T; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.0.next()?;
        let c = self.0.next()?;
        Some([a, b, c])
    }
}

impl<I, T> Iterator for Chunked<I, 4>
where
    I: Iterator<Item = T>,
{
    type Item = [T; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.0.next()?;
        let c = self.0.next()?;
        let d = self.0.next()?;
        Some([a, b, c, d])
    }
}
