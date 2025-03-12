// Answer 0

#[test]
fn test_raw_string_with_non_matching_delimiter_after_carriage_return() {
    let input_str = "Hello\rWorld\"Extra"; // This includes a carriage return followed by a quote.
    let cursor = Cursor { rest: input_str };
    let result = raw_string(cursor);
}

#[test]
fn test_raw_string_with_overlong_delimiter_following_carriage_return() {
    let input_str = "#This is a very long delimiter that exceeds the expected length which will trigger the Reject. Hello\rWorld\"Extra"; // Delimiter exceeds 255 characters.
    let cursor = Cursor { rest: input_str };
    let result = raw_string(cursor);
}

#[test]
fn test_raw_string_with_delimiter_check_failure_after_carriage_return() {
    let input_str = "SomeText\r\n\"NotTheExpectedDelimiter"; // The expected delimiter doesn't match.
    let cursor = Cursor { rest: input_str };
    let result = raw_string(cursor);
}

#[test]
fn test_raw_string_with_no_valid_delimiter_with_carriage_return() {
    let input_str = "#InvalidDelimiter\rAnythingAfter\"Invalid"; // Includes carriage return with a potential invalid delimiter.
    let cursor = Cursor { rest: input_str };
    let result = raw_string(cursor);
}

