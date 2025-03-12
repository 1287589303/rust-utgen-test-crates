// Answer 0

#[test]
fn test_cooked_c_string_with_carriage_return_no_newline() {
    let input_str = "Hello, world!\rA"; // Contains '\r' followed by 'A', no '\n' after
    let cursor = Cursor { rest: input_str };

    let _result = cooked_c_string(cursor);
}

#[test]
fn test_cooked_c_string_with_multiple_characters() {
    let input_str = "Sample text\rB"; // Contains '\r' followed by 'B', no '\n' after
    let cursor = Cursor { rest: input_str };
    
    let _result = cooked_c_string(cursor);
}

