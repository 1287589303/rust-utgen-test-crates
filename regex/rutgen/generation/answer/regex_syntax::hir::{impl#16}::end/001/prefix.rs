// Answer 0

#[test]
fn test_end_with_equal_chars() {
    let range = ClassUnicodeRange::new('a', 'a');
    let result = range.end();
}

#[test]
fn test_end_with_start_less_than_end() {
    let range = ClassUnicodeRange::new('a', 'z');
    let result = range.end();
}

#[test]
fn test_end_with_unicode_range() {
    let range = ClassUnicodeRange::new('\u{0000}', '\u{10FFFF}');
    let result = range.end();
}

#[test]
fn test_end_with_start_and_end_as_max_unicode() {
    let range = ClassUnicodeRange::new('\u{10FFFF}', '\u{10FFFF}');
    let result = range.end();
}

#[test]
fn test_end_with_start_equals_end_in_middle() {
    let range = ClassUnicodeRange::new('\u{1234}', '\u{1234}');
    let result = range.end();
}

