pub trait Integer: std::str::FromStr {}
impl Integer for u8 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for usize {}
impl Integer for i32 {}

pub fn to<T: Integer>(s: &str) -> T {
    match s.parse() {
        Ok(t) => t,
        Err(_) => panic!("Unable to parse \"{s}\""),
    }
}

pub fn to_unsigned_iter<'a, T>(s: &'a str) -> impl Iterator<Item = T> + 'a
where T: Integer + 'a
{
    fn not_numeric(c: char) -> bool {
        !c.is_ascii_digit()
    }

    fn not_empty(s: &&str) -> bool {
        !s.is_empty()
    }
    s.split(not_numeric).filter(not_empty).map(to::<T>)
}

pub fn to_vec<T: Integer>(s: &str) -> Vec<T> {
    to_unsigned_iter(s).collect()
}

pub fn to_tuple_1<T: Integer>(s: &str) -> T {
    let mut iter = to_unsigned_iter(s);
    iter.next().unwrap()
}

pub fn to_tuple_2<T: Integer>(s: &str) -> (T, T) {
    let mut iter = to_unsigned_iter(s);
    (iter.next().unwrap(), iter.next().unwrap())
}

pub fn to_tuple_3<T: Integer>(s: &str) -> (T, T, T) {
    let mut iter = to_unsigned_iter(s);
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

pub fn to_tuple_4<T: Integer>(s: &str) -> (T, T, T, T) {
    let mut iter = to_unsigned_iter(s);
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

pub fn to_signed_vec<T: Integer>(s: &str) -> Vec<T> {
    fn not_numeric(c: char) -> bool {
        !c.is_ascii_digit() && c != '-'
    }
    fn not_empty(s: &&str) -> bool {
        !s.is_empty()
    }
    s.split(not_numeric).filter(not_empty).map(to::<T>).collect()
}
