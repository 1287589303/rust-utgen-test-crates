// Answer 0

#[test]
fn test_unforce_with_wrap_proc_macro_enabled() {
    #[cfg(wrap_proc_macro)]
    {
        unforce();
    }
}

#[test]
fn test_unforce_with_wrap_proc_macro_disabled() {
    #[cfg(not(wrap_proc_macro))]
    {
        unforce();
    }
}

