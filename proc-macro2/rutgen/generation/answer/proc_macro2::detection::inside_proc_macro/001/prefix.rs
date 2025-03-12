// Answer 0

#[test]
fn test_inside_proc_macro_with_works_one() {
    WORKS.store(1, Ordering::Relaxed);
    let _result = inside_proc_macro();
}

#[test]
fn test_inside_proc_macro_with_works_two() {
    WORKS.store(2, Ordering::Relaxed);
    let _result = inside_proc_macro();
}

#[test]
fn test_inside_proc_macro_with_works_other() {
    WORKS.store(0, Ordering::Relaxed);
    let _result = inside_proc_macro();
}

#[test]
fn test_inside_proc_macro_with_works_greater_than_two() {
    WORKS.store(3, Ordering::Relaxed);
    let _result = inside_proc_macro();
}

