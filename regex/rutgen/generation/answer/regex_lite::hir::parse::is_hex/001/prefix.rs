// Answer 0

#[test]
fn test_is_hex_zero() {
    let c = '0';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_nine() {
    let c = '9';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_a() {
    let c = 'a';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_f() {
    let c = 'f';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_A() {
    let c = 'A';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_non_hex_g() {
    let c = 'g';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_non_hex_Z() {
    let c = 'Z';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_non_hex_at() {
    let c = '@';
    let result = is_hex(c);
}

#[test]
fn test_is_hex_whitespace() {
    let c = ' ';
    let result = is_hex(c);
}

