// Answer 0

#[test]
fn test_cooked_string_with_backslash_u() {
    let input = Cursor { rest: r#"some text with a backslash: \u "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_backslash_x() {
    let input = Cursor { rest: r#"some text with a backslash: \x "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_backslash_n() {
    let input = Cursor { rest: r#"some text with a backslash: \n "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_backslash_r() {
    let input = Cursor { rest: r#"some text with a backslash: \r "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_backslash_t() {
    let input = Cursor { rest: r#"some text with a backslash: \t "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_double_backslash() {
    let input = Cursor { rest: r#"some text with a backslash: "\\"# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_backslash_single_quote() {
    let input = Cursor { rest: r#"some text with a backslash: '\'' "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_backslash_double_quote() {
    let input = Cursor { rest: r#"some text with a backslash: "\"" "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_backslash_zero() {
    let input = Cursor { rest: r#"some text with a backslash: \0 "# };
    let _result = cooked_string(input);
}

#[test]
fn test_cooked_string_with_trailing_whitespace() {
    let input = Cursor { rest: r#"some text with a backslash: \u     "# };
    let _result = cooked_string(input);
}

