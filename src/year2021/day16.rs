//! Packet Decoder
//!
//! [`BitStream`] is the key to making this problem tractable. It works like an iterator, allowing
//! us to consume an arbitrary number of bits from the input and convert this to a number.
//!
//! It works by maintaining an internal `u64` buffer. If the requested number of bits is larger than
//! the buffer's current size then additional bits are added to the buffer 4 at a time from each
//! hexadecimal digit of the input data.
//!
//! Additionally it keeps track of the total number of bits consumed so far. This is needed when
//! parsing packets that use the total length in bits to determine sub-packets.
//!
//! The decoded packet data is stored as a tree-like struct allowing recursive solutions to part 1
//! and part 2 to reuse the same decoded input.
use std::str::Bytes;

struct BitStream<'a> {
    available: u64,
    bits: u64,
    read: u64,
    iter: Bytes<'a>,
}

impl BitStream<'_> {
    fn from(s: &str) -> BitStream<'_> {
        BitStream { available: 0, bits: 0, read: 0, iter: s.bytes() }
    }

    fn next(&mut self, amount: u64) -> u64 {
        while self.available < amount {
            self.available += 4;
            self.bits = (self.bits << 4) | self.hex_to_binary();
        }

        self.available -= amount;
        self.read += amount;

        let mask = (1 << amount) - 1;
        (self.bits >> self.available) & mask
    }

    fn hex_to_binary(&mut self) -> u64 {
        let hex_digit = self.iter.next().unwrap();

        if hex_digit.is_ascii_digit() { (hex_digit - 48) as u64 } else { (hex_digit - 55) as u64 }
    }
}

pub enum Packet {
    Literal { version: u64, type_id: u64, value: u64 },
    Operator { version: u64, type_id: u64, packets: Vec<Packet> },
}

impl Packet {
    fn from(bit_stream: &mut BitStream<'_>) -> Packet {
        let version = bit_stream.next(3);
        let type_id = bit_stream.next(3);

        if type_id == 4 {
            let mut todo = true;
            let mut value = 0;

            while todo {
                todo = bit_stream.next(1) == 1;
                value = (value << 4) | bit_stream.next(4);
            }

            Packet::Literal { version, type_id, value }
        } else {
            let mut packets = Vec::new();

            if bit_stream.next(1) == 0 {
                let target = bit_stream.next(15) + bit_stream.read;
                while bit_stream.read < target {
                    packets.push(Self::from(bit_stream));
                }
            } else {
                let sub_packets = bit_stream.next(11);
                for _ in 0..sub_packets {
                    packets.push(Self::from(bit_stream));
                }
            }

            Packet::Operator { version, type_id, packets }
        }
    }
}

pub fn parse(input: &str) -> Packet {
    let mut bit_stream = BitStream::from(input);
    Packet::from(&mut bit_stream)
}

pub fn part1(packet: &Packet) -> u64 {
    fn helper(packet: &Packet) -> u64 {
        match packet {
            Packet::Literal { version, .. } => *version,
            Packet::Operator { version, packets, .. } => {
                *version + packets.iter().map(helper).sum::<u64>()
            }
        }
    }

    helper(packet)
}

pub fn part2(packet: &Packet) -> u64 {
    fn helper(packet: &Packet) -> u64 {
        match packet {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { type_id, packets, .. } => {
                let mut iter = packets.iter().map(helper);
                match type_id {
                    0 => iter.sum(),
                    1 => iter.product(),
                    2 => iter.min().unwrap(),
                    3 => iter.max().unwrap(),
                    5 => (iter.next().unwrap() > iter.next().unwrap()) as u64,
                    6 => (iter.next().unwrap() < iter.next().unwrap()) as u64,
                    7 => (iter.next().unwrap() == iter.next().unwrap()) as u64,
                    _ => unreachable!(),
                }
            }
        }
    }

    helper(packet)
}
