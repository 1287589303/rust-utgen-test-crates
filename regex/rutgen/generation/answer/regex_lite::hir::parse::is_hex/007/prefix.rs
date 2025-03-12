// Answer 0

#[test]
fn test_is_hex_with_a() {
    let c: char = 'a';
    is_hex(c);
}

#[test]
fn test_is_hex_with_f() {
    let c: char = 'f';
    is_hex(c);
}

#[test]
fn test_is_hex_with_A() {
    let c: char = 'A';
    is_hex(c);
}

#[test]
fn test_is_hex_with_9() {
    let c: char = '9';
    is_hex(c);
}

#[test]
fn test_is_hex_with_0() {
    let c: char = '0';
    is_hex(c);
}

#[test]
fn test_is_hex_with_g() {
    let c: char = 'g';
    is_hex(c);
}

#[test]
fn test_is_hex_with_Z() {
    let c: char = 'Z';
    is_hex(c);
}

#[test]
fn test_is_hex_with_dash() {
    let c: char = '-';
    is_hex(c);
}

#[test]
fn test_is_hex_with_exclamation() {
    let c: char = '!';
    is_hex(c);
}

#[test]
fn test_is_hex_with_space() {
    let c: char = ' ';
    is_hex(c);
}

#[test]
fn test_is_hex_with_1() {
    let c: char = '1';
    is_hex(c);
}

