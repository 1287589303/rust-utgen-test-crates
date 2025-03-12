// Answer 0

#[test]
fn test_is_ascii_invalid_start_greater_than_end() {
    let range = ScalarRange { start: 5, end: 1 };
    range.is_ascii();
}

#[test]
fn test_is_ascii_invalid_end_exceeds_ascii() {
    let range = ScalarRange { start: 0x20, end: 0x80 };
    range.is_ascii();
}

#[test]
fn test_is_ascii_invalid_start_and_end_exceeding_ascii() {
    let range = ScalarRange { start: 0x81, end: 0xFF };
    range.is_ascii();
}

#[test]
fn test_is_ascii_valid_but_not_ascii() {
    let range = ScalarRange { start: 0x80, end: 0x10FFFF };
    range.is_ascii();
}

