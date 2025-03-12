// Answer 0

#[test]
fn test_c_string_with_cooked_input() {
    let input = Cursor { rest: "c\"valid string\"", off: 0 };
    let _ = c_string(input);
}

#[test]
fn test_c_string_with_raw_input() {
    let input = Cursor { rest: "crvalid raw string", off: 0 };
    let _ = c_string(input);
}

#[test]
fn test_c_string_with_invalid_input() {
    let input = Cursor { rest: "invalid string", off: 0 };
    let _ = c_string(input);
}

