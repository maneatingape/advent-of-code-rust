//! Utility methods to spawn a number of
//! [scoped](https://doc.rust-lang.org/stable/std/thread/fn.scope.html)
//! threads equals to the number of cores on the machine. Unlike normal threads, scoped threads
//! can borrow data from their environment.
use std::thread::*;

/// Spawn `n` scoped threads, where `n` is the available parallelism.
pub fn spawn<F, T>(f: F)
where
    F: FnOnce() -> T + Copy + Send,
    T: Send,
{
    scope(|scope| {
        for _ in 0..threads() {
            scope.spawn(f);
        }
    });
}

/// Splits `items` into batches, one per thread. Items are assigned in a round robin fashion,
/// to achieve a crude load balacing in case some items are more complex to process than others.
pub fn spawn_batches<F, T, U>(mut items: Vec<U>, f: F)
where
    F: FnOnce(Vec<U>) -> T + Copy + Send,
    T: Send,
    U: Send,
{
    let threads = threads();
    let mut batches: Vec<_> = (0..threads).map(|_| Vec::new()).collect();
    let mut index = 0;

    // Round robin items over each thread.
    while let Some(next) = items.pop() {
        batches[index % threads].push(next);
        index += 1;
    }

    scope(|scope| {
        for batch in batches {
            scope.spawn(move || f(batch));
        }
    });
}

fn threads() -> usize {
    available_parallelism().unwrap().get()
}
