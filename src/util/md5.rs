pub fn hash(message: &str) -> (u32, u32, u32, u32) {
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    let mut chunk: [u8; 64] = [0; 64];
    
    let raw = message.as_bytes();
    let len = raw.len();
    chunk[..len].copy_from_slice(&raw[..len]);
    
    chunk[len] = 0x80;
    let qux = (len * 8).to_le_bytes();
    chunk[56..64].copy_from_slice(&qux[..8]);

    let mut m: [u32; 16] = [0; 16];
    for i in 0..16 {
        let slice = &chunk[i*4..(i*4+4)];
        m[i] = u32::from_le_bytes(slice.try_into().unwrap());
    }    

    let mut a = a0;
    let mut b = b0;
    let mut c = c0;
    let mut d = d0;

    a = round1(a, b, c, d, m[ 0],  7, 0xd76aa478);
    d = round1(d, a, b, c, m[ 1], 12, 0xe8c7b756);
    c = round1(c, d, a, b, m[ 2], 17, 0x242070db);
    b = round1(b, c, d, a, m[ 3], 22, 0xc1bdceee);
    a = round1(a, b, c, d, m[ 4],  7, 0xf57c0faf);
    d = round1(d, a, b, c, m[ 5], 12, 0x4787c62a);
    c = round1(c, d, a, b, m[ 6], 17, 0xa8304613);
    b = round1(b, c, d, a, m[ 7], 22, 0xfd469501);
    a = round1(a, b, c, d, m[ 8],  7, 0x698098d8);
    d = round1(d, a, b, c, m[ 9], 12, 0x8b44f7af);
    c = round1(c, d, a, b, m[10], 17, 0xffff5bb1);
    b = round1(b, c, d, a, m[11], 22, 0x895cd7be);
    a = round1(a, b, c, d, m[12],  7, 0x6b901122);
    d = round1(d, a, b, c, m[13], 12, 0xfd987193);
    c = round1(c, d, a, b, m[14], 17, 0xa679438e);
    b = round1(b, c, d, a, m[15], 22, 0x49b40821);

    a = round2(a, b, c, d, m[ 1],  5, 0xf61e2562);
    d = round2(d, a, b, c, m[ 6],  9, 0xc040b340);
    c = round2(c, d, a, b, m[11], 14, 0x265e5a51);
    b = round2(b, c, d, a, m[ 0], 20, 0xe9b6c7aa);
    a = round2(a, b, c, d, m[ 5],  5, 0xd62f105d);
    d = round2(d, a, b, c, m[10],  9, 0x02441453);
    c = round2(c, d, a, b, m[15], 14, 0xd8a1e681);
    b = round2(b, c, d, a, m[ 4], 20, 0xe7d3fbc8);
    a = round2(a, b, c, d, m[ 9],  5, 0x21e1cde6);
    d = round2(d, a, b, c, m[14],  9, 0xc33707d6);
    c = round2(c, d, a, b, m[ 3], 14, 0xf4d50d87);
    b = round2(b, c, d, a, m[ 8], 20, 0x455a14ed);
    a = round2(a, b, c, d, m[13],  5, 0xa9e3e905);
    d = round2(d, a, b, c, m[ 2],  9, 0xfcefa3f8);
    c = round2(c, d, a, b, m[ 7], 14, 0x676f02d9);
    b = round2(b, c, d, a, m[12], 20, 0x8d2a4c8a);

    a = round3(a, b, c, d, m[ 5],  4, 0xfffa3942);
    d = round3(d, a, b, c, m[ 8], 11, 0x8771f681);
    c = round3(c, d, a, b, m[11], 16, 0x6d9d6122);
    b = round3(b, c, d, a, m[14], 23, 0xfde5380c);
    a = round3(a, b, c, d, m[ 1],  4, 0xa4beea44);
    d = round3(d, a, b, c, m[ 4], 11, 0x4bdecfa9);
    c = round3(c, d, a, b, m[ 7], 16, 0xf6bb4b60);
    b = round3(b, c, d, a, m[10], 23, 0xbebfbc70);
    a = round3(a, b, c, d, m[13],  4, 0x289b7ec6);
    d = round3(d, a, b, c, m[ 0], 11, 0xeaa127fa);
    c = round3(c, d, a, b, m[ 3], 16, 0xd4ef3085);
    b = round3(b, c, d, a, m[ 6], 23, 0x04881d05);
    a = round3(a, b, c, d, m[ 9],  4, 0xd9d4d039);
    d = round3(d, a, b, c, m[12], 11, 0xe6db99e5);
    c = round3(c, d, a, b, m[15], 16, 0x1fa27cf8);
    b = round3(b, c, d, a, m[ 2], 23, 0xc4ac5665);

    a = round4(a, b, c, d, m[ 0],  6, 0xf4292244);
    d = round4(d, a, b, c, m[ 7], 10, 0x432aff97);
    c = round4(c, d, a, b, m[14], 15, 0xab9423a7);
    b = round4(b, c, d, a, m[ 5], 21, 0xfc93a039);
    a = round4(a, b, c, d, m[12],  6, 0x655b59c3);
    d = round4(d, a, b, c, m[ 3], 10, 0x8f0ccc92);
    c = round4(c, d, a, b, m[10], 15, 0xffeff47d);
    b = round4(b, c, d, a, m[ 1], 21, 0x85845dd1);
    a = round4(a, b, c, d, m[ 8],  6, 0x6fa87e4f);
    d = round4(d, a, b, c, m[15], 10, 0xfe2ce6e0);
    c = round4(c, d, a, b, m[ 6], 15, 0xa3014314);
    b = round4(b, c, d, a, m[13], 21, 0x4e0811a1);
    a = round4(a, b, c, d, m[ 4],  6, 0xf7537e82);
    d = round4(d, a, b, c, m[11], 10, 0xbd3af235);
    c = round4(c, d, a, b, m[ 2], 15, 0x2ad7d2bb);
    b = round4(b, c, d, a, m[ 9], 21, 0xeb86d391);

    a0 = a0.wrapping_add(a);
    b0 = b0.wrapping_add(b);
    c0 = c0.wrapping_add(c);
    d0 = d0.wrapping_add(d);

    (a0.to_be(), b0.to_be(), c0.to_be(), d0.to_be())
}

fn round1(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & c) | (!b & d);
    common(f, a, b, m, s, k)
}

fn round2(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & d) | (c & !d);
    common(f, a, b, m, s, k)
}

fn round3(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = b ^ c ^ d;
    common(f, a, b, m, s, k)
}

fn round4(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = c ^ (b | !d);
    common(f, a, b, m, s, k)
}

fn common(f: u32, a: u32, b: u32, m: u32, s: u32, k: u32) -> u32 {
    f.wrapping_add(a).wrapping_add(k).wrapping_add(m).rotate_left(s).wrapping_add(b)
}
