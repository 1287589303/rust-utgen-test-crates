// Answer 0

#[test]
fn test_literal_suffix_valid_identifier() {
    let input = Cursor {
        rest: "abc123_def$ghi",
        #[cfg(span_locations)]
        off: 0,
    };
    literal_suffix(input);
}

#[test]
fn test_literal_suffix_valid_identifier_with_special_character() {
    let input = Cursor {
        rest: "$validIdentifier123",
        #[cfg(span_locations)]
        off: 0,
    };
    literal_suffix(input);
}

#[test]
fn test_literal_suffix_valid_identifier_with_underscore() {
    let input = Cursor {
        rest: "_leadingUnderscore",
        #[cfg(span_locations)]
        off: 0,
    };
    literal_suffix(input);
}

#[test]
fn test_literal_suffix_valid_identifier_with_multiple_continuation_chars() {
    let input = Cursor {
        rest: "A1B2C3_$D4E5F6G7",
        #[cfg(span_locations)]
        off: 0,
    };
    literal_suffix(input);
}

#[test]
fn test_literal_suffix_just_above_boundary() {
    let input = Cursor {
        rest: "a",
        #[cfg(span_locations)]
        off: 0,
    };
    literal_suffix(input);
}

