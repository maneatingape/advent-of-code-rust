//! [Min heap] more suitable for algorithms such as [Dijkstra] and [A*] than Rust's default
//! max heap. Splits the sorting key and value, so that you can order items without having
//! to implement [`Ord`] on the value type.
//!
//! [Min heap]: https://en.wikipedia.org/wiki/Heap_(data_structure)
//! [Dijkstra]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
//! [A*]: https://en.wikipedia.org/wiki/A*_search_algorithm
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Wrapper<K: Ord, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for Wrapper<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for Wrapper<K, V> {}

impl<K: Ord, V> PartialOrd for Wrapper<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Wrapper<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key)
    }
}

#[derive(Default)]
pub struct MinHeap<K: Ord, V> {
    heap: BinaryHeap<Wrapper<K, V>>,
}

impl<K: Ord, V> MinHeap<K, V> {
    pub fn new() -> Self {
        MinHeap { heap: BinaryHeap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MinHeap { heap: BinaryHeap::with_capacity(capacity) }
    }

    #[inline]
    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(Wrapper { key, value });
    }

    #[inline]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|w| (w.key, w.value))
    }

    #[inline]
    pub fn peek(&self) -> Option<(&K, &V)> {
        self.heap.peek().map(|w| (&w.key, &w.value))
    }
}
