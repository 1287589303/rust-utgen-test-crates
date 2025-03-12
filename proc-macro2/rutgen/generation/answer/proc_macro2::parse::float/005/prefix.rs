// Answer 0

#[test]
fn test_float_with_valid_float_and_non_ident_start_char() {
    let cursor = Cursor { rest: "3.14a" };
    let result = float(cursor);
}

#[test]
fn test_float_with_valid_float_and_non_ident_start_char_exponent() {
    let cursor = Cursor { rest: "2.7e10" };
    let result = float(cursor);
}

#[test]
fn test_float_with_valid_float_with_underscore_and_non_ident_start_char() {
    let cursor = Cursor { rest: "1_000.25b" };
    let result = float(cursor);
}

#[test]
fn test_float_with_valid_float_and_non_ident_start_special_char() {
    let cursor = Cursor { rest: "0.99!" };
    let result = float(cursor);
}

#[test]
fn test_float_with_valid_float_with_multiple_parts_and_non_ident_start_char() {
    let cursor = Cursor { rest: "5.0e+3x" };
    let result = float(cursor);
}

