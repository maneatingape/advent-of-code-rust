pub trait VecExt<T> {
    fn fill(n: usize, t: T) -> Vec<T> where T: Copy;
    fn tabulate(n: usize, f: impl Fn(usize) -> T) -> Vec<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn fill(n: usize, t: T) -> Vec<T> where T: Copy {
        let mut vec: Vec<T> = Vec::with_capacity(n);
        (0..n).for_each(|_| vec.push(t));
        vec
    }

    fn tabulate(n: usize, f: impl Fn(usize) -> T) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(n);
        (0..n).for_each(|i| vec.push(f(i)));
        vec
    }
}

pub struct Tupled2<I>(I);
pub struct Tupled3<I>(I);
pub struct Tupled4<I>(I);

pub trait Tupled where Self: Sized {
    fn tupled2(self) -> Tupled2<Self>;
    fn tupled3(self) -> Tupled3<Self>;
    fn tupled4(self) -> Tupled4<Self>;
}

impl<I> Tupled for I where I: Iterator {
    fn tupled2(self) -> Tupled2<Self> {
        Tupled2(self)
    }

    fn tupled3(self) -> Tupled3<Self> {
        Tupled3(self)
    }

    fn tupled4(self) -> Tupled4<Self> {
        Tupled4(self)
    }
}

impl<I, T> Iterator for Tupled2<I> where I: Iterator<Item = T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.0.next()?;
        Some((a, b))
    }
}

impl<I, T> Iterator for Tupled3<I> where I: Iterator<Item = T> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.0.next()?;
        let c = self.0.next()?;
        Some((a, b, c))
    }
}

impl<I, T> Iterator for Tupled4<I> where I: Iterator<Item = T> {
    type Item = (T, T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.0.next()?;
        let c = self.0.next()?;
        let d = self.0.next()?;
        Some((a, b, c, d))
    }
}
