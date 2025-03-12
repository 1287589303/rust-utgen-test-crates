// Answer 0

#[test]
fn test_parse_valid_tag() {
    let cursor = Cursor {
        rest: "hello world",
        #[cfg(span_locations)]
        off: 0,
    };
    let tag = "hello";
    let result = cursor.parse(tag);
}

#[test]
fn test_parse_tag_at_start() {
    let cursor = Cursor {
        rest: "rust programming",
        #[cfg(span_locations)]
        off: 0,
    };
    let tag = "rust";
    let result = cursor.parse(tag);
}

#[test]
fn test_parse_partial_tag() {
    let cursor = Cursor {
        rest: "foo bar baz",
        #[cfg(span_locations)]
        off: 0,
    };
    let tag = "foo";
    let result = cursor.parse(tag);
}

#[test]
fn test_parse_tag_equal_to_rest() {
    let cursor = Cursor {
        rest: "exact",
        #[cfg(span_locations)]
        off: 0,
    };
    let tag = "exact";
    let result = cursor.parse(tag);
}

#[test]
fn test_parse_tag_with_whitespace() {
    let cursor = Cursor {
        rest: "leading whitespace",
        #[cfg(span_locations)]
        off: 0,
    };
    let tag = "leading";
    let result = cursor.parse(tag);
}

