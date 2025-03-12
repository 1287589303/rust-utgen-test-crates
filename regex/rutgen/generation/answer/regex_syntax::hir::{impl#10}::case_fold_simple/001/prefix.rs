// Answer 0

#[test]
fn test_case_fold_simple_with_ascii_range() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange::new(b'A', b'Z'),
    ]));
    class_bytes.case_fold_simple();
}

#[test]
#[should_panic]
fn test_case_fold_simple_with_unicode_class() {
    let mut class_unicode = Class::Unicode(ClassUnicode::new(vec![
        ClassUnicodeRange::new('A', 'Z'),
    ]));
    class_unicode.case_fold_simple();
}

