// Answer 0

#[test]
fn test_int_with_valid_hexadecimal_and_identifier_start() {
    let cursor = Cursor { rest: "0x1A _" };
    let result = int(cursor);
}

#[test]
fn test_int_with_valid_hexadecimal_and_identifier_start_at_end() {
    let cursor = Cursor { rest: "0x1A" };
    let result = int(cursor);
}

#[test]
fn test_int_with_valid_octal_and_identifier_start() {
    let cursor = Cursor { rest: "0o77 _" };
    let result = int(cursor);
}

#[test]
fn test_int_with_valid_octal_and_identifier_start_at_end() {
    let cursor = Cursor { rest: "0o77" };
    let result = int(cursor);
}

#[test]
fn test_int_with_valid_binary_and_identifier_start() {
    let cursor = Cursor { rest: "0b1010 _" };
    let result = int(cursor);
}

#[test]
fn test_int_with_valid_binary_and_identifier_start_at_end() {
    let cursor = Cursor { rest: "0b1010" };
    let result = int(cursor);
}

