// Answer 0

#[test]
fn test_contains_lower_bound_control() {
    let ascii_set = AsciiSet::EMPTY.union(*CONTROLS);
    let byte: u8 = 0;
    ascii_set.contains(byte);
}

#[test]
fn test_contains_upper_bound_control() {
    let ascii_set = AsciiSet::EMPTY.union(*CONTROLS);
    let byte: u8 = 127;
    ascii_set.contains(byte);
}

#[test]
fn test_contains_non_alphanumeric_space() {
    let ascii_set = AsciiSet::EMPTY.union(*NON_ALPHANUMERIC);
    let byte: u8 = b' ';
    ascii_set.contains(byte);
}

#[test]
fn test_contains_non_alphanumeric_exclamation() {
    let ascii_set = AsciiSet::EMPTY.union(*NON_ALPHANUMERIC);
    let byte: u8 = b'!';
    ascii_set.contains(byte);
}

#[test]
fn test_contains_non_alphanumeric_tilde() {
    let ascii_set = AsciiSet::EMPTY.union(*NON_ALPHANUMERIC);
    let byte: u8 = b'~';
    ascii_set.contains(byte);
}

#[test]
fn test_contains_valid_alphanumeric_a() {
    let ascii_set = AsciiSet::EMPTY;
    let byte: u8 = b'a';
    ascii_set.contains(byte);
}

#[test]
fn test_contains_valid_alphanumeric_1() {
    let ascii_set = AsciiSet::EMPTY;
    let byte: u8 = b'1';
    ascii_set.contains(byte);
}

#[test]
fn test_contains_invalid_character_high() {
    let ascii_set = AsciiSet::EMPTY;
    let byte: u8 = 128;
    ascii_set.contains(byte);
}

