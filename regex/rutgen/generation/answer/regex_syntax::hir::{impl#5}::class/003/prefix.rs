// Answer 0

#[test]
fn test_class_with_non_empty_unicode_literal() {
    let unicode_class = Class::Unicode(ClassUnicode::from_literal(vec![b'a', b'b', b'c']));
    let hir = Hir::class(unicode_class);
}

#[test]
fn test_class_with_non_empty_bytes_literal() {
    let bytes_class = Class::Bytes(ClassBytes::from_literal(vec![b'x', b'y', b'z']));
    let hir = Hir::class(bytes_class);
}

