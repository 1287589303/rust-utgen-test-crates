// Answer 0

#[test]
fn test_punct_char_valid_input_1() {
    let cursor = Cursor { rest: "!example", off: 0 };
    let _ = punct_char(cursor);
}

#[test]
fn test_punct_char_valid_input_2() {
    let cursor = Cursor { rest: "@test", off: 0 };
    let _ = punct_char(cursor);
}

#[test]
fn test_punct_char_valid_input_3() {
    let cursor = Cursor { rest: "#hash", off: 0 };
    let _ = punct_char(cursor);
}

#[test]
fn test_punct_char_valid_input_4() {
    let cursor = Cursor { rest: "$money", off: 0 };
    let _ = punct_char(cursor);
}

#[test]
fn test_punct_char_valid_input_5() {
    let cursor = Cursor { rest: "%percent", off: 0 };
    let _ = punct_char(cursor);
}

