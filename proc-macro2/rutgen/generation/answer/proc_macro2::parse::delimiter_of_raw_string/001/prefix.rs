// Answer 0

#[test]
fn test_delimiter_of_raw_string_with_valid_input() {
    let input = Cursor {
        rest: "##\"valid\"",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_with_first_quote() {
    let input = Cursor {
        rest: "\"start",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_with_multi_hashes() {
    let input = Cursor {
        rest: "#####\"end",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_with_exceeding_length() {
    let input = Cursor {
        rest: "###".repeat(100) + "\"more",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_with_non_hash_and_quote() {
    let input = Cursor {
        rest: "##a\"middle",
        #[cfg(span_locations)]
        off: 0,
    };
    let _ = delimiter_of_raw_string(input);
}

