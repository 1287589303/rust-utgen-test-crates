// Answer 0

#[test]
fn test_cooked_c_string_with_escape_sequence_zero() {
    let input = Cursor {
        rest: "\\u0000", 
        off: 0,
    };
    let _result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_u_invalid() {
    let input = Cursor {
        rest: "\\uXYZ", 
        off: 0,
    };
    let _result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_trailing_backslash_newline() {
    let input = Cursor {
        rest: "\\\n", 
        off: 0,
    };
    let _result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_trailing_backslash_returning_newline() {
    let input = Cursor {
        rest: "\\\r", 
        off: 0,
    };
    let _result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_nonvalid_escape() {
    let input = Cursor {
        rest: "\\u", 
        off: 0,
    };
    let _result = cooked_c_string(input);
}

