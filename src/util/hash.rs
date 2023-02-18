// Simplified implementation of the fast Rust C
// hash algorithm, also used by Firefox.
// https://github.com/rust-lang/rustc-hash
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hasher};

pub type FastSet<T> = HashSet<T, BuildFxHasher>;
pub type FastMap<T> = HashMap<T, BuildFxHasher>;

pub struct FastSetBuilder;

impl FastSetBuilder {
    pub fn new<T>() -> FastSet<T> {
        HashSet::with_hasher(BuildFxHasher)
    }

    pub fn with_capacity<T>(capacity: usize) -> FastSet<T> {
        HashSet::with_capacity_and_hasher(capacity, BuildFxHasher)
    }
}

#[derive(Clone, Copy, Default)]
pub struct BuildFxHasher;

impl BuildHasher for BuildFxHasher {
    type Hasher = FxHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FxHasher { hash: 0 }
    }
}

pub struct FxHasher {
    hash: u64,
}

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: u64) {
        self.hash = (self.hash.rotate_left(5) ^ i).wrapping_mul(0x517cc1b727220a95);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            self.add_to_hash(u64::from_ne_bytes(bytes[..8].try_into().unwrap()) as u64);
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            self.add_to_hash(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as u64);
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.add_to_hash(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as u64);
            bytes = &bytes[2..];
        }
        if bytes.len() >= 1 {
            self.add_to_hash(bytes[0] as u64);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}
