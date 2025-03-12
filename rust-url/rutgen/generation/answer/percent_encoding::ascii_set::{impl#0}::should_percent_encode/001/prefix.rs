// Answer 0

#[test]
fn test_should_percent_encode_ascii_control_characters() {
    let ascii_set = *CONTROLS;
    let result = ascii_set.should_percent_encode(0x00);
}

#[test]
fn test_should_percent_encode_ascii_visible_characters() {
    let ascii_set = *NON_ALPHANUMERIC;
    let result = ascii_set.should_percent_encode(b'!');
}

#[test]
fn test_should_percent_encode_ascii_uppercase() {
    let ascii_set = *NON_ALPHANUMERIC;
    let result = ascii_set.should_percent_encode(b'A');
}

#[test]
fn test_should_percent_encode_ascii_lowercase() {
    let ascii_set = *NON_ALPHANUMERIC;
    let result = ascii_set.should_percent_encode(b'z');
}

#[test]
fn test_should_percent_encode_ascii_boundary() {
    let ascii_set = *NON_ALPHANUMERIC;
    let result = ascii_set.should_percent_encode(0x7F);
}

