// Answer 0

#[test]
fn test_once_box_default_creation() {
    let once_box: OnceBox<i32> = OnceBox::default();
    let inner_pointer = once_box.inner.load(Ordering::SeqCst);
    assert_eq!(inner_pointer, ptr::null_mut());
}

#[test]
fn test_once_box_default_creation_other_type() {
    let once_box: OnceBox<String> = OnceBox::default();
    let inner_pointer = once_box.inner.load(Ordering::SeqCst);
    assert_eq!(inner_pointer, ptr::null_mut());
}

#[test]
fn test_once_box_default_creation_with_custom_type() {
    let once_box: OnceBox<f64> = OnceBox::default();
    let inner_pointer = once_box.inner.load(Ordering::SeqCst);
    assert_eq!(inner_pointer, ptr::null_mut());
}

