// Answer 0

#[test]
fn test_digits_hex_boundary_1() {
    let input = Cursor { rest: "0x10" };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_empty_1() {
    let input = Cursor { rest: "0x0" };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_with_underscore_1() {
    let input = Cursor { rest: "0x1_2" };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_with_underscore_2() {
    let input = Cursor { rest: "0x10_1" };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_invalid_character() {
    let input = Cursor { rest: "0xgg" };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_long_input() {
    let input = Cursor { rest: "0x1234567890abcdef" };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_mixed_underscore() {
    let input = Cursor { rest: "0x1_2a_3" };
    let _ = digits(input);
}

#[test]
fn test_digits_hex_multiple_underscores() {
    let input = Cursor { rest: "0x1_2_3" };
    let _ = digits(input);
}

