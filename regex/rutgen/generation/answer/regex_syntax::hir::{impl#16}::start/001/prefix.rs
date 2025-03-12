// Answer 0

#[test]
fn test_start_valid_range() {
    let range = ClassUnicodeRange::new('a', 'z');
    let start = range.start();
}

#[test]
fn test_start_equal_bounds() {
    let range = ClassUnicodeRange::new('a', 'a');
    let start = range.start();
}

#[test]
fn test_start_control_character() {
    let range = ClassUnicodeRange::new('\u{0000}', '\u{0001}');
    let start = range.start();
}

#[test]
fn test_start_upper_unicode_limit() {
    let range = ClassUnicodeRange::new('\u{FFFE}', '\u{FFFF}');
    let start = range.start();
}

#[test]
fn test_start_full_unicode_range() {
    let range = ClassUnicodeRange::new('\u{0000}', '\u{10FFFF}');
    let start = range.start();
}

