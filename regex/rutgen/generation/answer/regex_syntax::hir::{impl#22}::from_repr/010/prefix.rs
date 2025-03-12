// Answer 0

#[test]
fn test_from_repr_valid_word_unicode_negate() {
    let repr: u32 = 0b00_0000_0010_0000_0000; // 32
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_unicode() {
    let repr: u32 = 0b00_0000_0001_0000_0000; // 256
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_ascii() {
    let repr: u32 = 0b00_0000_0000_0100_0000; // 64
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_endlf() {
    let repr: u32 = 0b00_0000_0000_0000_1000; // 8
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_end() {
    let repr: u32 = 0b00_0000_0000_0000_0010; // 2
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_start() {
    let repr: u32 = 0b00_0000_0000_0000_0001; // 1
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_startlf() {
    let repr: u32 = 0b00_0000_0000_0000_0100; // 4
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_startcrlf() {
    let repr: u32 = 0b00_0000_0000_0001_0000; // 16
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_ascii_negate() {
    let repr: u32 = 0b00_0000_0000_1000_0000; // 128
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_start_ascii() {
    let repr: u32 = 0b00_0000_0100_0000_0000; // 4096
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_end_ascii() {
    let repr: u32 = 0b00_0000_1000_0000_0000; // 8192
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_start_unicode() {
    let repr: u32 = 0b00_0001_0000_0000_0000; // 1024
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_end_unicode() {
    let repr: u32 = 0b00_0010_0000_0000_0000; // 2048
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_start_half_ascii() {
    let repr: u32 = 0b00_0100_0000_0000_0000; // 4096
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_end_half_ascii() {
    let repr: u32 = 0b00_1000_0000_0000_0000; // 8192
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_start_half_unicode() {
    let repr: u32 = 0b01_0000_0000_0000_0000; // 16384
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_word_end_half_unicode() {
    let repr: u32 = 0b10_0000_0000_0000_0000; // 32768
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr: u32 = 0b0000_0000_0000_0000_0000; // 0
    let result = Look::from_repr(repr);
}

