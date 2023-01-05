pub fn to_i32(string: &str) -> i32 {
    string.parse().unwrap()
}

pub fn to_u32(string: &str) -> u32 {
    string.parse().unwrap()
}

pub fn to_u32_3(s: &str) -> [u32; 3] {
    let mut iter = s.split(not_numeric).filter(not_empty).map(to_u32);
    [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()]
}

pub fn to_u32_4(s: &str) -> [u32; 4] {
    let mut iter = s.split(not_numeric).filter(not_empty).map(to_u32);
    [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()]
}

fn not_numeric(c: char) -> bool {
    !c.is_ascii_digit()
}

fn not_empty(s: &&str) -> bool {
    !s.is_empty()
}
