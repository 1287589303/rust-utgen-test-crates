// Answer 0

#[test]
fn test_class_with_empty_unicode() {
    let empty_unicode_class = Class::Unicode(ClassUnicode::empty());
    let _result = Hir::class(empty_unicode_class);
}

#[test]
fn test_class_with_empty_bytes() {
    let empty_bytes_class = Class::Bytes(ClassBytes::empty());
    let _result = Hir::class(empty_bytes_class);
}

