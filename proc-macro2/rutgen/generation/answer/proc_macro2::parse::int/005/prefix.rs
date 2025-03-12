// Answer 0

#[test]
fn test_int_with_decimal_digits_followed_by_non_ident_start_char() {
    let cursor = Cursor { rest: "123a" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_hex_digits_followed_by_non_ident_start_char() {
    let cursor = Cursor { rest: "0x1A!" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_octal_digits_followed_by_non_ident_start_char() {
    let cursor = Cursor { rest: "0o755%" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_binary_digits_followed_by_non_ident_start_char() {
    let cursor = Cursor { rest: "0b101#" };
    let _ = int(cursor);
}

#[test]
fn test_int_with_mixed_decimal_with_non_ident_start_char() {
    let cursor = Cursor { rest: "123abc" };
    let _ = int(cursor);
}

