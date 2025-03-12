use core::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
static WORKS: AtomicUsize = AtomicUsize::new(0);
static INIT: Once = Once::new();
pub(crate) fn force_fallback() {
    WORKS.store(1, Ordering::Relaxed);
}
