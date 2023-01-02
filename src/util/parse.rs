pub fn to_i32(string: &&str) -> i32 {
    string.parse().unwrap()
}

pub fn to_vec_u32(string: &str) -> Vec<u32> {
    string
        .split(|c| !char::is_numeric(c))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
