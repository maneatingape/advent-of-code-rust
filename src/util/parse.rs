use std::fmt::Debug;
use std::str::FromStr;

pub fn to<T>(s: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.parse().unwrap()
}

pub fn to_iter<'a, T>(s: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + 'a,
    <T as FromStr>::Err: Debug,
{
    s.split(not_numeric).filter(not_empty).map(to::<T>)
}

pub fn to_vec<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    to_iter(s).collect()
}

pub fn to_array1<T>(s: &str) -> [T; 1]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut iter = to_iter(s);
    [iter.next().unwrap()]
}

pub fn to_array2<T>(s: &str) -> [T; 2]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut iter = to_iter(s);
    [iter.next().unwrap(), iter.next().unwrap()]
}

pub fn to_array3<T>(s: &str) -> [T; 3]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut iter = to_iter(s);
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

pub fn to_array4<T>(s: &str) -> [T; 4]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut iter = to_iter(s);
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn not_numeric(c: char) -> bool {
    !c.is_ascii_digit()
}

fn not_empty(s: &&str) -> bool {
    !s.is_empty()
}
