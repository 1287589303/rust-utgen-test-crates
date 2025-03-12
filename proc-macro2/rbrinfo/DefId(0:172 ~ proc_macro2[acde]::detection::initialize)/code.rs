fn initialize() {
    let available = proc_macro::is_available();
    WORKS.store(available as usize + 1, Ordering::Relaxed);
}