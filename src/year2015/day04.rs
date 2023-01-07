use crate::util::md5::hash;
use std::fmt::Write;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    find(input, 0xfffff000)
}

pub fn part2(input: &str) -> u32 {
    find(input, 0xffffff00)
}

fn find(input: &str, mask: u32) -> u32 {
    let mut index = 0;
    let mut s = String::with_capacity(32);
    let mut a = 0x12345678;

    while (a & mask) != 0 {
        index += 1;
        s.clear();
        write!(&mut s, "{input}{index}").expect("Not enough space");
        a = hash(&s).0;
    }

    index
}
