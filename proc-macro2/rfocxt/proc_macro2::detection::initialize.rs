use core::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
static WORKS: AtomicUsize = AtomicUsize::new(0);
static INIT: Once = Once::new();
#[cfg(no_is_available)]
fn initialize() {
    use std::panic::{self, PanicInfo};
    type PanicHook = dyn Fn(&PanicInfo) + Sync + Send + 'static;
    let null_hook: Box<PanicHook> = Box::new(|_panic_info| {});
    let sanity_check = &*null_hook as *const PanicHook;
    let original_hook = panic::take_hook();
    panic::set_hook(null_hook);
    let works = panic::catch_unwind(proc_macro::Span::call_site).is_ok();
    WORKS.store(works as usize + 1, Ordering::Relaxed);
    let hopefully_null_hook = panic::take_hook();
    panic::set_hook(original_hook);
    if sanity_check != &*hopefully_null_hook {
        panic!("observed race condition in proc_macro2::inside_proc_macro");
    }
}
