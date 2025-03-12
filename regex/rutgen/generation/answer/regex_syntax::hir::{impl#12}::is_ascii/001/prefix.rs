// Answer 0

#[test]
fn test_is_ascii_empty() {
    let class_unicode = ClassUnicode::empty();
    class_unicode.is_ascii();
}

#[test]
fn test_is_ascii_only_ascii() {
    let ascii_range = vec![ClassUnicodeRange { start: 'a', end: 'z' }];
    let class_unicode = ClassUnicode::new(ascii_range);
    class_unicode.is_ascii();
}

#[test]
fn test_is_ascii_with_non_ascii() {
    let mixed_range = vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: '¡', end: '¢' },
    ];
    let class_unicode = ClassUnicode::new(mixed_range);
    class_unicode.is_ascii();
}

#[test]
fn test_is_ascii_mixed_range() {
    let mixed_range = vec![
        ClassUnicodeRange { start: '\0', end: '\x7F' }, // ASCII
        ClassUnicodeRange { start: 'Ž', end: 'ž' },    // Non-ASCII
    ];
    let class_unicode = ClassUnicode::new(mixed_range);
    class_unicode.is_ascii();
}

#[test]
fn test_is_ascii_full_range() {
    let full_range = vec![
        ClassUnicodeRange { start: '\0', end: '\u{10FFFF}' },
    ];
    let class_unicode = ClassUnicode::new(full_range);
    class_unicode.is_ascii();
}

