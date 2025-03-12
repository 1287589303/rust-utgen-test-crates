// Answer 0

#[test]
fn test_once_box_new() {
    let once_box: OnceBox<u32> = OnceBox::new();
    let inner = once_box.inner.load(Ordering::Relaxed);
    assert_eq!(inner, ptr::null_mut());
}

#[test]
fn test_once_box_new_empty() {
    let once_box: OnceBox<String> = OnceBox::new();
    let inner = once_box.inner.load(Ordering::Relaxed);
    assert_eq!(inner, ptr::null_mut());
}

#[test]
fn test_once_box_new_for_custom_type() {
    struct CustomType {
        value: i32,
    }
    let once_box: OnceBox<CustomType> = OnceBox::new();
    let inner = once_box.inner.load(Ordering::Relaxed);
    assert_eq!(inner, ptr::null_mut());
}

