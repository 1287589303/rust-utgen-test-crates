// Answer 0

#[test]
fn test_byte_string_cooked_case() {
    let input = Cursor { rest: "b\"\\x41\"" }; // starts with "b\"" and has a valid byte escaping
    let _ = byte_string(input);
}

#[test]
fn test_byte_string_cooked_case_no_br_after_prefix() {
    let input = Cursor { rest: "b\"Hello \\n World\"" }; // starts with "b\"" and has valid escaping, but no "br" immediately after
    let _ = byte_string(input);
}

#[test]
fn test_byte_string_cooked_case_valid_escape() {
    let input = Cursor { rest: "b\"\\\"test\\\" end\"" }; // starts with "b\"" and has a valid escape immediately after
    let _ = byte_string(input);
}

