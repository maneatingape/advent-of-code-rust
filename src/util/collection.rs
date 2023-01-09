pub trait VecOps1<T> {
    fn fill(n: usize, t: T) -> Vec<T>;
}

impl<T: Copy> VecOps1<T> for Vec<T> {
    fn fill(n: usize, t: T) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(n);
        (0..n).for_each(|_| vec.push(t));
        vec
    }
}

pub trait VecOps2<T> {
    fn tabulate(n: usize, f: impl Fn(usize) -> T) -> Vec<T>;
}

impl<T> VecOps2<T> for Vec<T> {
    fn tabulate(n: usize, f: impl Fn(usize) -> T) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(n);
        (0..n).for_each(|i| vec.push(f(i)));
        vec
    }
}
