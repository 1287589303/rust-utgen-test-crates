// Answer 0

#[test]
fn test_unicode_is_empty_empty() {
    let unicode_class = Class::Unicode(ClassUnicode::empty());
    unicode_class.is_empty();
}

#[test]
fn test_unicode_is_empty_one_range() {
    let unicode_class = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'a' }]));
    unicode_class.is_empty();
}

#[test]
fn test_unicode_is_empty_multiple_empty_ranges() {
    let unicode_class = Class::Unicode(ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'd', end: 'c' }, // Represents an empty range
        ClassUnicodeRange { start: 'f', end: 'e' }  // Represents another empty range
    ]));
    unicode_class.is_empty();
}

#[test]
fn test_bytes_is_empty_empty() {
    let bytes_class = Class::Bytes(ClassBytes::empty());
    bytes_class.is_empty();
}

#[test]
fn test_bytes_is_empty_one_range() {
    let bytes_class = Class::Bytes(ClassBytes::new(vec![ClassBytesRange { start: 1, end: 1 }]));
    bytes_class.is_empty();
}

#[test]
fn test_bytes_is_empty_multiple_empty_ranges() {
    let bytes_class = Class::Bytes(ClassBytes::new(vec![
        ClassBytesRange { start: 3, end: 2 }, // Represents an empty range
        ClassBytesRange { start: 5, end: 4 }  // Represents another empty range
    ]));
    bytes_class.is_empty();
}

