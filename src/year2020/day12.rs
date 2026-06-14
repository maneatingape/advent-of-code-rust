//! # Rain Risk
//!
//! On this problem parsing takes almost all the time, so for maximum speed
//! a custom parser solves both parts during a single pass over the input bytes
//!

type Input = (i32,i32);

const DX:[i32;4] = [1,0,-1,0];
const DY:[i32;4] = [0,1,0,-1];

pub fn parse(input: &str) -> Input {
    let mut inp = input.to_string();
    if inp.as_bytes()[inp.len()-1] != b'\n' { inp.push('\n');}
    let bytes = inp.as_bytes();
    let mut i = 0;
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut direction:usize = 0;
    let mut x2:i32 = 0;
    let mut y2:i32 = 0;
    let mut wx:i32 = 10; // Waypoint starts at E10, N1
    let mut wy:i32 = -1;

    while i < bytes.len() {
        let cmd = bytes[i]; i+= 1;
        let mut n:i32 = (bytes[i] - b'0') as i32; i += 1;
        while bytes[i] != b'\n' {
            n = n*10 + (bytes[i] - b'0') as i32; i += 1;
        }
        i += 1;  // skip the newline
        match cmd {
            b'N' => {y -= n; wy -= n;},
            b'S' => {y += n; wy += n;},
            b'E' => {x += n; wx += n;},
            b'W' => {x -= n; wx -= n;},
            b'L' => {
                let mut ddir = n/90; 
                direction = (direction + 4 - ddir as usize) & 3;
                while ddir > 0 {
                    (wx,wy) = (wy,-wx);
                    ddir -= 1;
                }
            },
            b'R' => {
                let mut ddir = n/90; 
                direction = (direction + ddir as usize) & 3;
                while ddir > 0 {
                    (wx,wy) = (-wy,wx);
                    ddir -= 1;
                }
            },
            b'F' => {
                x += DX[direction]*n; 
                y += DY[direction]*n;
                x2 += wx*n;
                y2 += wy*n;
            },
            _ => {panic!("Bad command {cmd}")},
        }
    }
    let part1 = x.abs() + y.abs();
    let part2 = x2.abs() + y2.abs();
    (part1,part2)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

