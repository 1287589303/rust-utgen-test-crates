// Answer 0

#[test]
fn test_parse_with_non_matching_tag_1() {
    let cursor = Cursor { rest: "example string" };
    let result = cursor.parse("test");
}

#[test]
fn test_parse_with_non_matching_tag_2() {
    let cursor = Cursor { rest: "another example" };
    let result = cursor.parse("example");
}

#[test]
fn test_parse_with_non_matching_tag_empty() {
    let cursor = Cursor { rest: "non-empty" };
    let result = cursor.parse("");
}

#[test]
fn test_parse_with_non_matching_tag_longer_than_rest() {
    let cursor = Cursor { rest: "" };
    let result = cursor.parse("longtag");
}

