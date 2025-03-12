// Answer 0

#[test]
fn test_len_identical_start_end() {
    let range = ClassUnicodeRange::new('\u{0000}', '\u{0000}');
    range.len();
}

#[test]
fn test_len_minimum_range() {
    let range = ClassUnicodeRange::new('\u{0041}', '\u{0041}'); // 'A'
    range.len();
}

#[test]
fn test_len_small_range() {
    let range = ClassUnicodeRange::new('\u{0041}', '\u{0043}'); // 'A' to 'C'
    range.len();
}

#[test]
fn test_len_large_range() {
    let range = ClassUnicodeRange::new('\u{0000}', '\u{FFFF}'); // Full range
    range.len();
}

#[should_panic]
fn test_len_invalid_range() {
    let range = ClassUnicodeRange::new('\u{FFFF}', '\u{0000}'); // Invalid range
    range.len();
}

