// Answer 0

#[test]
fn test_float_valid_float_followed_by_invalid_identifier() {
    let input_str = "3.14a"; // Valid float representation followed by an identifier start character
    let cursor = Cursor { rest: input_str };

    let _ = float(cursor);
}

#[test]
fn test_float_valid_float_with_e_followed_by_invalid_identifier() {
    let input_str = "1.0e+3b"; // Valid float representation with exponential part followed by an identifier start character
    let cursor = Cursor { rest: input_str };

    let _ = float(cursor);
}

#[test]
fn test_float_float_with_multiple_decimals_followed_by_invalid_identifier() {
    let input_str = "2.3.4a"; // Invalid float representation with multiple decimals followed by an identifier start character
    let cursor = Cursor { rest: input_str };

    let _ = float(cursor);
}

#[test]
fn test_float_float_with_invalid_exp_part() {
    let input_str = "5.67e+g"; // Valid float representation with invalid exponential character followed by identifier start character
    let cursor = Cursor { rest: input_str };

    let _ = float(cursor);
}

#[test]
fn test_float_float_with_invalid_character_after_valid_float() {
    let input_str = "0.99b"; // Valid float representation followed by an invalid character
    let cursor = Cursor { rest: input_str };

    let _ = float(cursor);
}

