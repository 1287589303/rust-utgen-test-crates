pub fn force() {
    #[cfg(wrap_proc_macro)]
    crate::detection::force_fallback();
}