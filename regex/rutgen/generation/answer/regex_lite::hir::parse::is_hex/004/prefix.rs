// Answer 0

#[test]
fn test_is_hex_with_zero() {
    let c = '0';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_nine() {
    let c = '9';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_a() {
    let c = 'a';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_f() {
    let c = 'f';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_A() {
    let c = 'A';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_F() {
    let c = 'F';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_non_hex_lowercase() {
    let c = 'g';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_non_hex_uppercase() {
    let c = '@';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_special_character() {
    let c = '*';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_with_one() {
    let c = '1';
    let result = is_hex(c);
}

