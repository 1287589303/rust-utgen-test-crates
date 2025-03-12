// Answer 0

#[test]
fn test_ident_any_with_invalid_start_char_digit() {
    let input = Cursor { rest: "1abc", off: 0 };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_invalid_start_char_special() {
    let input = Cursor { rest: "!", off: 0 };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_invalid_start_char_space() {
    let input = Cursor { rest: " abc", off: 0 };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_empty_string() {
    let input = Cursor { rest: "", off: 0 };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_invalid_start_char_digit_offset() {
    let input = Cursor { rest: "1abc", off: 5 };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_invalid_start_char_special_offset() {
    let input = Cursor { rest: "!", off: 10 };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_invalid_start_space_offset() {
    let input = Cursor { rest: " abc", off: 7 };
    let result = ident_any(input);
}

#[test]
fn test_ident_any_with_empty_string_offset() {
    let input = Cursor { rest: "", off: 3 };
    let result = ident_any(input);
}

