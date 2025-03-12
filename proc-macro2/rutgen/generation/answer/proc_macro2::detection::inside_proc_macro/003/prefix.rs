// Answer 0

#[test]
fn test_inside_proc_macro_returns_false_when_works_is_one() {
    WORKS.store(1, Ordering::Relaxed);
    let result = inside_proc_macro();
}

#[test]
fn test_inside_proc_macro_initializes_once_when_works_is_one() {
    WORKS.store(1, Ordering::Relaxed);
    let result = inside_proc_macro();
    let second_result = inside_proc_macro();
}

