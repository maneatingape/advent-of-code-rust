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
