// Answer 0

#[test]
fn test_cooked_c_string_with_backslash_u_invalid() {
    let input = Cursor { rest: "\\u" };
    let result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_x() {
    let input = Cursor { rest: "\\x" };
    let result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_trailing_backslash_newline() {
    let input = Cursor { rest: "\\\n" };
    let result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_n() {
    let input = Cursor { rest: "\\n" };
    let result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_r() {
    let input = Cursor { rest: "\\r" };
    let result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_t() {
    let input = Cursor { rest: "\\t" };
    let result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_single_quote() {
    let input = Cursor { rest: "\\'" };
    let result = cooked_c_string(input);
}

#[test]
fn test_cooked_c_string_with_backslash_double_quote() {
    let input = Cursor { rest: "\\" };
    let result = cooked_c_string(input);
}

