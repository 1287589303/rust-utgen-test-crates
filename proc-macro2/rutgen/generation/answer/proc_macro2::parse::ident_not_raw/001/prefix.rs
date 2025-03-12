// Answer 0

#[test]
fn test_ident_not_raw_empty_string() {
    let cursor = Cursor { rest: "" };
    let result = ident_not_raw(cursor);
}

#[test]
fn test_ident_not_raw_invalid_start_character() {
    let cursor = Cursor { rest: "123abc" }; // starts with a digit
    let result = ident_not_raw(cursor);
}

#[test]
fn test_ident_not_raw_special_character() {
    let cursor = Cursor { rest: "@invalid" }; // starts with a special character
    let result = ident_not_raw(cursor);
}

#[test]
fn test_ident_not_raw_single_special_character() {
    let cursor = Cursor { rest: "!" }; // only special character
    let result = ident_not_raw(cursor);
}

