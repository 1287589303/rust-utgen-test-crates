// Answer 0

#[test]
fn test_delimiter_of_raw_string_with_more_than_256_hashes() {
    let input = Cursor {
        rest: "#####################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_with_256_hashes() {
    let input = Cursor {
        rest: "#####################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################################",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(input);
}

#[test]
fn test_delimiter_of_raw_string_with_mixed_bytes_not_hash_or_quote() {
    let input = Cursor {
        rest: "abcde",
        #[cfg(span_locations)]
        off: 0,
    };
    let _result = delimiter_of_raw_string(input);
}

