pub(crate) fn force_fallback() {
    WORKS.store(1, Ordering::Relaxed);
}