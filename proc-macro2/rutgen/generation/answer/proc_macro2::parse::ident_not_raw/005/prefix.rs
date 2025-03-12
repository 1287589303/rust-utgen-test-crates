// Answer 0

#[test]
fn test_ident_not_raw_err_when_is_ident_start_false() {
    let cursor = Cursor { rest: "_1abc".to_string().as_str() };
    let result = ident_not_raw(cursor);
}

#[test]
fn test_ident_not_raw_err_with_special_characters() {
    let cursor = Cursor { rest: "a!bc".to_string().as_str() };
    let result = ident_not_raw(cursor);
}

#[test]
fn test_ident_not_raw_err_with_numeric_start() {
    let cursor = Cursor { rest: "1abc".to_string().as_str() };
    let result = ident_not_raw(cursor);
}

#[test]
fn test_ident_not_raw_err_with_whitespace() {
    let cursor = Cursor { rest: " abc".to_string().as_str() };
    let result = ident_not_raw(cursor);
}

#[test]
fn test_ident_not_raw_err_with_invalid_start() {
    let cursor = Cursor { rest: "$identifier".to_string().as_str() };
    let result = ident_not_raw(cursor);
}

