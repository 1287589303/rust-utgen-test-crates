use core::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
static WORKS: AtomicUsize = AtomicUsize::new(0);
static INIT: Once = Once::new();
pub(crate) fn inside_proc_macro() -> bool {
    match WORKS.load(Ordering::Relaxed) {
        1 => return false,
        2 => return true,
        _ => {}
    }
    INIT.call_once(initialize);
    inside_proc_macro()
}
