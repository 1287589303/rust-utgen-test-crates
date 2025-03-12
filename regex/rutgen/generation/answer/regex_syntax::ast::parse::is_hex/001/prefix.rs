// Answer 0

#[test]
fn test_is_hex_with_zero() {
    let result = is_hex('0');
}

#[test]
fn test_is_hex_with_nine() {
    let result = is_hex('9');
}

#[test]
fn test_is_hex_with_a() {
    let result = is_hex('a');
}

#[test]
fn test_is_hex_with_f() {
    let result = is_hex('f');
}

#[test]
fn test_is_hex_with_A() {
    let result = is_hex('A');
}

#[test]
fn test_is_hex_with_F() {
    let result = is_hex('F');
}

#[test]
fn test_is_hex_with_non_hexadecimal_g() {
    let result = is_hex('g');
}

#[test]
fn test_is_hex_with_non_hexadecimal_Z() {
    let result = is_hex('Z');
}

#[test]
fn test_is_hex_with_space() {
    let result = is_hex(' ');
}

#[test]
fn test_is_hex_with_at_symbol() {
    let result = is_hex('@');
}

