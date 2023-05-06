use crate::util::md5::hash;

pub fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    find(input, 0xfffff000)
}

pub fn part2(input: &[u8]) -> u32 {
    find(input, 0xffffff00)
}

fn find(input: &[u8], mask: u32) -> u32 {
    let mut count = 0;
    let mut size = input.len();
    let mut buffer = [b'0'; 32];

    buffer[..size].copy_from_slice(input);

    loop {
        count += 1;

        let mut index = size;
        loop {
            if buffer[index] < b'9' {
                buffer[index] += 1;
                break;
            } else if index == input.len() {
                buffer[index] = b'1';
                size += 1;
                break;
            } else {
                buffer[index] = b'0';
                index -= 1;
            }
        }

        if hash(&buffer[..(size + 1)]).0 & mask == 0 {
            return count;
        }
    }
}
