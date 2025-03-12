// Answer 0

#[test]
fn test_next_valid_split_and_ascii() {
    let mut utf8_sequences = Utf8Sequences::new('\0', '\u{D7FF}');
    let result = utf8_sequences.next();
}

#[test]
fn test_next_valid_range() {
    let mut utf8_sequences = Utf8Sequences::new('\u{0080}', '\u{D7FF}');
    let result = utf8_sequences.next();
}

#[test]
fn test_next_with_invalid_start_greater_than_max() {
    let mut utf8_sequences = Utf8Sequences::new('\u{D800}', '\u{D7FF}');
    let result = utf8_sequences.next();
}

#[test]
fn test_next_with_valid_ascii_range() {
    let mut utf8_sequences = Utf8Sequences::new('\u{0000}', '\u{007F}');
    let result = utf8_sequences.next();
}

#[test]
fn test_next_with_one_byte_encoding() {
    let mut utf8_sequences = Utf8Sequences::new('\u{80}', '\u{7FF}');
    let result = utf8_sequences.next();
}

#[test]
fn test_next_with_two_byte_encoding() {
    let mut utf8_sequences = Utf8Sequences::new('\u{800}', '\u{FFFF}');
    let result = utf8_sequences.next();
}

#[test]
fn test_next_with_three_byte_encoding() {
    let mut utf8_sequences = Utf8Sequences::new('\u{10000}', '\u{1FFFF}');
    let result = utf8_sequences.next();
}

#[test]
fn test_next_with_four_byte_encoding() {
    let mut utf8_sequences = Utf8Sequences::new('\u{200000}', '\u{10FFFF}');
    let result = utf8_sequences.next();
}

