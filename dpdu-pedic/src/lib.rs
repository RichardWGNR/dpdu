use std::ops::Deref;

mod dpdu;
mod utils;
mod passthru;

fn main() {
    println!("Hello, world!");
}

/// Marks the wrapped value as `Send` and `Sync`.
///
/// # Safety
///
/// The caller must ensure that sharing or transferring `T` across threads
/// is sound. Wrapping a non-thread-safe type in `SendSync` can cause
/// undefined behavior.
#[derive(Debug, Clone)]
pub(crate) struct SendSync<T>(pub T);

unsafe impl<T> Send for SendSync<T> {}
unsafe impl<T> Sync for SendSync<T> {}

impl<T> Deref for SendSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}