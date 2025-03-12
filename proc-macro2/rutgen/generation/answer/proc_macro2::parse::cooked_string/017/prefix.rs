// Answer 0

#[test]
fn test_cooked_string_unsupported_sequence() {
    let input = Cursor { 
        rest: "\\g", 
        #[cfg(span_locations)]
        off: 0 
    };
    let _ = cooked_string(input);
}

#[test]
fn test_cooked_string_multiple_unsupported_sequences() {
    let input = Cursor { 
        rest: "\\g\\z", 
        #[cfg(span_locations)]
        off: 0 
    };
    let _ = cooked_string(input);
}

#[test]
fn test_cooked_string_leading_backslash_with_non_supported_sequence() {
    let input = Cursor { 
        rest: "\\m", 
        #[cfg(span_locations)]
        off: 0 
    };
    let _ = cooked_string(input);
}

#[test]
fn test_cooked_string_backslash_escape_sequence_with_newline() {
    let input = Cursor { 
        rest: "\\n\\g", 
        #[cfg(span_locations)]
        off: 0 
    };
    let _ = cooked_string(input);
}

