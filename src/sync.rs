#[cfg(loom)]
pub(crate) use loom::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
#[cfg(loom)]
pub(crate) use mutex::*;
#[cfg(loom)]
mod mutex {
    use core::ops::{Deref, DerefMut};
    pub(crate) use loom::sync::Arc;
    pub(crate) use loom::sync::MutexGuard;

    #[derive(Debug, Default)]
    pub(crate) struct Mutex<T>(loom::sync::Mutex<T>);

    impl<T> Mutex<T> {
        pub fn lock(&self) -> MutexGuard<'_, T> {
            self.0.lock().unwrap()
        }
    }
}

#[cfg(loom)]
pub(crate) fn fence(ord: Ordering) {
    if let Ordering::Acquire = ord {
    } else {
        // FIXME: loom only supports acquire fences at the moment.
        // https://github.com/tokio-rs/loom/issues/117
        // let's at least not panic...
        // this may generate some false positives (`SeqCst` is stronger than `Acquire`
        // for example), and some false negatives (`Relaxed` is weaker than `Acquire`),
        // but it's the best we can do for the time being.
    }
    loom::sync::atomic::fence(Ordering::Acquire)
}

#[cfg(not(loom))]
pub(crate) use alloc::sync::Arc;
#[cfg(not(loom))]
pub(crate) use core::sync::atomic::{fence, AtomicPtr, AtomicUsize, Ordering};

#[cfg(all(not(loom), not(feature = "std")))]
pub(crate) use spin::{Mutex, MutexGuard};

#[cfg(all(not(loom), feature = "std"))]
pub(crate) use spin::{Mutex, MutexGuard};
