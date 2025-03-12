// Answer 0

#[test]
fn test_is_passthrough_ascii_label_empty_label() {
    let label: &[u8] = &[];
    is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_short_label() {
    let label: &[u8] = b"ab"; // Length < 4
    is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_valid_first_invalid_last() {
    let label: &[u8] = b"abc-"; // Length >= 4, first is valid, last is invalid
    is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_valid_first_invalid_tail() {
    let label: &[u8] = b"ade-1"; // Length >= 4, first valid, one tail invalid
    is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_valid_first_invalid_tail_two_invalid() {
    let label: &[u8] = b"ade--1"; // Length >= 4, first valid, multiple tails invalid
    is_passthrough_ascii_label(label);
}

