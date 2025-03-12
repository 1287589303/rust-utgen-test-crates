// Answer 0

#[test]
fn test_push_valid_range() {
    let mut class_unicode = ClassUnicode::empty();
    let range = ClassUnicodeRange { start: 'a', end: 'z' };
    class_unicode.push(range);
}

#[test]
fn test_push_empty_class_unicode() {
    let mut class_unicode = ClassUnicode::empty();
    // no action taken as it's empty
    let range = ClassUnicodeRange { start: '0', end: '0' };
    class_unicode.push(range);
}

#[test]
fn test_push_overlapping_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'b', end: 'd' },
    ]);
    let range = ClassUnicodeRange { start: 'c', end: 'e' };
    class_unicode.push(range);
}

#[test]
fn test_push_adjacent_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
    ]);
    let range = ClassUnicodeRange { start: 'c', end: 'c' };
    class_unicode.push(range);
}

#[test]
fn test_push_unicode_block_ranges() {
    let mut class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange { start: '\u{0041}', end: '\u{005A}' }, // A-Z
        ClassUnicodeRange { start: '\u{0061}', end: '\u{007A}' }, // a-z
    ]);
    let range = ClassUnicodeRange { start: '\u{0391}', end: '\u{0396}' }; // Greek capital letters
    class_unicode.push(range);
}

