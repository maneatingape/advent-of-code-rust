//! Provides fast [`HashSet`] and [`HashMap`] implementations based on a simplified implementation of
//! the fast [Rust C hash algorithm](https://github.com/rust-lang/rustc-hash) also used by
//! [Firefox](https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html).
//
//! By default, Rust's [`HashMap`] and [`HashSet`] use a [DDoS](https://en.wikipedia.org/wiki/Denial-of-service_attack)
//! resistant but slower hashing algorithm. [`FxHasher`] is much faster (between 2x to 5x from my testing).
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};

/// Type alias for [`HashSet`] using [`FxHasher`].
pub type FastSet<T> = HashSet<T, BuildFxHasher>;
/// Type alias for [`HashMap`] using [`FxHasher`].
pub type FastMap<K, V> = HashMap<K, V, BuildFxHasher>;

/// Convenience methods to contruct a [`FastSet`].
pub trait FastSetBuilder<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> FastSet<T>;
}

impl<T> FastSetBuilder<T> for FastSet<T> {
    fn new() -> Self {
        HashSet::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> FastSet<T> {
        HashSet::with_capacity_and_hasher(capacity, BuildFxHasher)
    }
}

/// Convenience methods to contruct a [`FastMap`].
pub trait FastMapBuilder<K, V> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn build<const N: usize>(array: [(K, V); N]) -> Self;
}

impl<K: Eq + Hash, V> FastMapBuilder<K, V> for FastMap<K, V> {
    fn new() -> Self {
        HashMap::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, BuildFxHasher)
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

    fn build_hasher(&self) -> Self::Hasher {
        FxHasher { state: 0 }
    }
}

/// Simplified implementation, in particular running on a system with 64 bit `usize` is assumed.
///
/// Checkout the [Firefox code](https://searchfox.org/mozilla-central/rev/633345116df55e2d37be9be6555aa739656c5a7d/mfbt/HashFunctions.h#109-153)
/// for a full description.
pub struct FxHasher {
    state: u64,
}

impl FxHasher {
    #[inline]
    fn hash(&mut self, i: u64) {
        self.state = (self.state.rotate_left(5) ^ i).wrapping_mul(0x517cc1b727220a95);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            self.hash(u64::from_ne_bytes(bytes[..8].try_into().unwrap()));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            self.hash(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as u64);
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.hash(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as u64);
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            self.hash(bytes[0] as u64);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.state
    }
}
