// Answer 0

#[test]
fn test_punct_char_invalid_start() {
    let input = Cursor { rest: "abc", off: 0 };
    let _result = punct_char(input);
}

#[test]
fn test_punct_char_invalid_start_special_char() {
    let input = Cursor { rest: "%", off: 0 };
    let _result = punct_char(input);
}

#[test]
fn test_punct_char_invalid_start_empty_comment() {
    let input = Cursor { rest: "abc/* comment */", off: 0 };
    let _result = punct_char(input);
}

#[test]
fn test_punct_char_invalid_start_slash() {
    let input = Cursor { rest: "*/", off: 0 };
    let _result = punct_char(input);
}

#[test]
fn test_punct_char_invalid_start_text_before() {
    let input = Cursor { rest: "text@moretext", off: 0 };
    let _result = punct_char(input);
}

