// Answer 0

#[test]
fn test_once_ref_default() {
    let instance: OnceRef<u32> = OnceRef::default();
}

#[test]
fn test_once_ref_new() {
    let instance: OnceRef<i32> = OnceRef::new();
}

