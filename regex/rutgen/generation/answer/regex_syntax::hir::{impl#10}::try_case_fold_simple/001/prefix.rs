// Answer 0

#[test]
fn test_try_case_fold_simple_bytes() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0x41, 0x5A), // A-Z
        ClassBytesRange::new(0x61, 0x7A), // a-z
    ]));
    class_bytes.try_case_fold_simple().unwrap();
}

#[test]
fn test_try_case_fold_simple_empty_bytes() {
    let mut class_bytes = Class::Bytes(ClassBytes::empty());
    class_bytes.try_case_fold_simple().unwrap();
}

#[test]
fn test_try_case_fold_simple_single_range() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0x41, 0x41), // A
    ]));
    class_bytes.try_case_fold_simple().unwrap();
}

#[test]
fn test_try_case_fold_simple_large_range() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0x20, 0x7E), // Space to ~
    ]));
    class_bytes.try_case_fold_simple().unwrap();
}

#[test]
fn test_try_case_fold_simple_non_ascii() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(0xFF, 0xFF), // Non-ASCII
    ]));
    class_bytes.try_case_fold_simple().unwrap();
}

