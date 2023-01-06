pub trait VecOps<T> {
    fn tabulate(n: usize, f: impl Fn(usize) -> T) -> Vec<T>;
}

impl<T> VecOps<T> for Vec<T> {
    fn tabulate(n: usize, f: impl Fn(usize) -> T) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(n);
        (0..n).for_each(|i| vec.push(f(i)));
        vec
    }
}
