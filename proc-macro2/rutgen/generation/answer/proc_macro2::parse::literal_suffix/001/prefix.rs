// Answer 0

#[test]
fn test_literal_suffix_invalid_start() {
    let input = Cursor { rest: "123abc", off: 0 };
    let _result = literal_suffix(input);
}

#[test]
fn test_literal_suffix_invalid_start_with_special_char() {
    let input = Cursor { rest: "!@#abc", off: 0 };
    let _result = literal_suffix(input);
}

#[test]
fn test_literal_suffix_invalid_start_with_space() {
    let input = Cursor { rest: " abc", off: 0 };
    let _result = literal_suffix(input);
}

#[test]
fn test_literal_suffix_invalid_start_with_empty_string() {
    let input = Cursor { rest: "", off: 0 };
    let _result = literal_suffix(input);
}

#[test]
fn test_literal_suffix_invalid_start_with_numbers_only() {
    let input = Cursor { rest: "789", off: 0 };
    let _result = literal_suffix(input);
}

