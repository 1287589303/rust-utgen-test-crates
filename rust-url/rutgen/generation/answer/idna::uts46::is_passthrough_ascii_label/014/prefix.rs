// Answer 0

#[test]
fn test_is_passthrough_ascii_label_valid() {
    let label: &[u8] = &[b'a', b'1', b'-', b'b'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_invalid_length() {
    let label: &[u8] = &[b'a', b'1', b'-', b'b'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_invalid_hyphen() {
    let label: &[u8] = &[b'a', b'1', b'-', b'-'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_invalid_first_char() {
    let label: &[u8] = &[b'A', b'1', b'-', b'b'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_invalid_last_char() {
    let label: &[u8] = &[b'a', b'1', b'-', b'-'];
    let result = is_passthrough_ascii_label(label);
}

