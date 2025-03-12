// Answer 0

#[test]
fn test_cooked_string_with_valid_u_char() {
    let input = Cursor {
        rest: "\\u0041".into(), 
        off: 0,
    };
    let _ = cooked_string(input);
}

#[test]
fn test_cooked_string_with_valid_x_char() {
    let input = Cursor {
        rest: "\\x41".into(), 
        off: 0,
    };
    let _ = cooked_string(input);
}

#[test]
fn test_cooked_string_with_newline_after_backslash() {
    let input = Cursor {
        rest: "\\\n".into(), 
        off: 0,
    };
    let _ = cooked_string(input);
}

#[test]
fn test_cooked_string_with_valid_escape_sequence() {
    let input = Cursor {
        rest: "\\n".into(), 
        off: 0,
    };
    let _ = cooked_string(input);
}

#[test]
fn test_cooked_string_with_mixed_escape_sequence() {
    let input = Cursor {
        rest: "\\u0041\\n".into(), 
        off: 0,
    };
    let _ = cooked_string(input);
}

