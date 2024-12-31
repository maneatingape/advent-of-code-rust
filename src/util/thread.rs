//! Utility methods to spawn a number of
//! [scoped](https://doc.rust-lang.org/stable/std/thread/fn.scope.html)
//! threads equals to the number of cores on the machine. Unlike normal threads, scoped threads
//! can borrow data from their environment.
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::thread::*;

// Usually the number of physical cores.
fn threads() -> usize {
    available_parallelism().unwrap().get()
}

/// Spawn `n` scoped threads, where `n` is the available parallelism.
pub fn spawn<F>(f: F)
where
    F: Fn() + Copy + Send,
{
    scope(|scope| {
        for _ in 0..threads() {
            scope.spawn(f);
        }
    });
}

/// Spawns `n` scoped threads that each receive a
/// [work stealing](https://en.wikipedia.org/wiki/Work_stealing) iterator.
/// Work stealing is an efficient strategy that keeps each CPU core busy when some items take longer
/// than other to process, used by popular libraries such as [rayon](https://github.com/rayon-rs/rayon).
/// Processing at different rates also happens on many modern CPUs with
/// [heterogeneous performance and efficiency cores](https://en.wikipedia.org/wiki/ARM_big.LITTLE).
pub fn spawn_parallel_iterator<F, T>(items: &[T], f: F)
where
    F: Fn(ParIter<'_, T>) + Copy + Send,
    T: Sync,
{
    let threads = threads();
    let size = items.len().div_ceil(threads);

    // Initially divide work as evenly as possible amongst each worker thread.
    let workers: Vec<_> = (0..threads)
        .map(|id| {
            let start = (id * size).min(items.len());
            let end = (start + size).min(items.len());
            CachePadding::new(pack(start, end))
        })
        .collect();
    let workers = workers.as_slice();

    scope(|scope| {
        for id in 0..threads {
            scope.spawn(move || f(ParIter { id, items, workers }));
        }
    });
}

pub struct ParIter<'a, T> {
    id: usize,
    items: &'a [T],
    workers: &'a [CachePadding],
}

impl<'a, T> Iterator for ParIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // First try taking from our own queue.
        let worker = &self.workers[self.id];
        let current = worker.increment();
        let (start, end) = unpack(current);

        // There's still items to process.
        if start < end {
            return Some(&self.items[start]);
        }

        // Steal from another worker, [spinlocking](https://en.wikipedia.org/wiki/Spinlock)
        // until we acquire new items to process or there's nothing left to do.
        loop {
            // Find worker with the most remaining items.
            let available = self
                .workers
                .iter()
                .filter_map(|other| {
                    let current = other.load();
                    let (start, end) = unpack(current);
                    let size = end.saturating_sub(start);

                    (size > 0).then_some((other, current, size))
                })
                .max_by_key(|t| t.2);

            if let Some((other, current, size)) = available {
                // Split the work items into two roughly equal piles.
                let (start, end) = unpack(current);
                let middle = start + size.div_ceil(2);

                let next = pack(middle, end);
                let stolen = pack(start + 1, middle);

                // We could be preempted by another thread stealing or by the owning worker
                // thread finishing an item, so check indices are still unmodified.
                if other.compare_exchange(current, next) {
                    worker.store(stolen);
                    break Some(&self.items[start]);
                }
            } else {
                // No work remaining.
                break None;
            }
        }
    }
}

/// Intentionally force alignment to 128 bytes to make a best effort attempt to place each atomic
/// on its own cache line. This reduces contention and improves performance for common
/// CPU caching protocols such as [MESI](https://en.wikipedia.org/wiki/MESI_protocol).
#[repr(align(128))]
pub struct CachePadding {
    atomic: AtomicUsize,
}

/// Convenience wrapper methods around atomic operations. Both start and end indices are packed
/// into a single atomic so that we can use the fastest and easiest to reason about `Relaxed`
/// ordering.
impl CachePadding {
    #[inline]
    fn new(n: usize) -> Self {
        CachePadding { atomic: AtomicUsize::new(n) }
    }

    #[inline]
    fn increment(&self) -> usize {
        self.atomic.fetch_add(1, Relaxed)
    }

    #[inline]
    fn load(&self) -> usize {
        self.atomic.load(Relaxed)
    }

    #[inline]
    fn store(&self, n: usize) {
        self.atomic.store(n, Relaxed);
    }

    #[inline]
    fn compare_exchange(&self, current: usize, new: usize) -> bool {
        self.atomic.compare_exchange(current, new, Relaxed, Relaxed).is_ok()
    }
}

#[inline]
fn pack(start: usize, end: usize) -> usize {
    (end << 32) | start
}

#[inline]
fn unpack(both: usize) -> (usize, usize) {
    (both & 0xffffffff, both >> 32)
}
