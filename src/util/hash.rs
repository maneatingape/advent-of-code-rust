//! Provides fast [`HashSet`] and [`HashMap`] implementations based on a simplified implementation of
//! the fast [Rust C hash algorithm](https://github.com/rust-lang/rustc-hash) also used by
//! [Firefox](https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html).
//
//! By default, Rust's [`HashMap`] and [`HashSet`] use a [DDoS](https://en.wikipedia.org/wiki/Denial-of-service_attack)
//! resistant but slower hashing algorithm. [`FxHasher`] is much faster (between 2x to 5x from my testing).
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::BitXor as _;

/// Type alias for [`HashSet`] using [`FxHasher`].
pub type FastSet<T> = HashSet<T, BuildFxHasher>;

/// Convenience methods to construct a [`FastSet`].
pub trait FastSetBuilder<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn build<const N: usize>(array: [T; N]) -> Self;
}

impl<T: Eq + Hash> FastSetBuilder<T> for FastSet<T> {
    fn new() -> Self {
        Self::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }

    fn build<const N: usize>(array: [T; N]) -> Self {
        let mut set = Self::new();
        set.extend(array);
        set
    }
}

/// Type alias for [`HashMap`] using [`FxHasher`].
pub type FastMap<K, V> = HashMap<K, V, BuildFxHasher>;

/// Convenience methods to construct a [`FastMap`].
pub trait FastMapBuilder<K, V> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn build<const N: usize>(array: [(K, V); N]) -> Self;
}

impl<K: Eq + Hash, V> FastMapBuilder<K, V> for FastMap<K, V> {
    fn new() -> Self {
        Self::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }

    fn build<const N: usize>(array: [(K, V); N]) -> Self {
        let mut map = Self::new();
        map.extend(array);
        map
    }
}

/// If you want an instance of [`FxHasher`] then this has you covered.
#[derive(Clone, Copy, Default)]
pub struct BuildFxHasher;

impl BuildHasher for BuildFxHasher {
    type Hasher = FxHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FxHasher { hash: 0 }
    }
}

/// Simplified implementation, in particular running on a system with 64 bit `usize` is assumed.
///
/// Check out the [Firefox code](https://searchfox.org/mozilla-central/rev/633345116df55e2d37be9be6555aa739656c5a7d/mfbt/HashFunctions.h#109-153)
/// for a full description.
const K: u64 = 0x517cc1b727220a95;

pub struct FxHasher {
    hash: u64,
}

impl FxHasher {
    #[inline]
    fn add(&mut self, i: u64) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            self.add(u64::from_ne_bytes(bytes[..8].try_into().unwrap()));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            self.add(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as u64);
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.add(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as u64);
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            self.add(bytes[0] as u64);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}
