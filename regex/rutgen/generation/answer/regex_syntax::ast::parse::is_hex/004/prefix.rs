// Answer 0

#[test]
fn test_is_hex_with_zero() {
    let c = '0';
    is_hex(c);
}

#[test]
fn test_is_hex_with_nine() {
    let c = '9';
    is_hex(c);
}

#[test]
fn test_is_hex_with_a() {
    let c = 'a';
    is_hex(c);
}

#[test]
fn test_is_hex_with_f() {
    let c = 'f';
    is_hex(c);
}

#[test]
fn test_is_hex_with_A() {
    let c = 'A';
    is_hex(c);
}

#[test]
fn test_is_hex_with_F() {
    let c = 'F';
    is_hex(c);
}

#[test]
fn test_is_hex_with_G() {
    let c = 'G';
    is_hex(c);
}

#[test]
fn test_is_hex_with_Z() {
    let c = 'Z';
    is_hex(c);
}

#[test]
fn test_is_hex_with_exclamation() {
    let c = '!';
    is_hex(c);
}

#[test]
fn test_is_hex_with_space() {
    let c = ' ';
    is_hex(c);
}

#[test]
fn test_is_hex_with_control_character() {
    let c = '\n';
    is_hex(c);
}

