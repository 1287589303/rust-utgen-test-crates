// Answer 0

#[test]
fn test_int_hex_ident() {
    let input = Cursor { rest: "0x1aA_" };
    let _ = int(input);
}

#[test]
fn test_int_octal_ident() {
    let input = Cursor { rest: "0o57a" };
    let _ = int(input);
}

#[test]
fn test_int_binary_ident() {
    let input = Cursor { rest: "0b1010_" };
    let _ = int(input);
}

#[test]
fn test_int_decimal_ident() {
    let input = Cursor { rest: "1234_" };
    let _ = int(input);
}

#[test]
fn test_int_hex_ident_no_raw() {
    let input = Cursor { rest: "0xF1n" };
    let _ = int(input);
}

