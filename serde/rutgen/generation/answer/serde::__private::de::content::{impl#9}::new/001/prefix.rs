// Answer 0

#[test]
fn test_new_with_valid_static_strings() {
    let visitor = TaggedContentVisitor::new("valid_tag", "expecting_value");
}

#[test]
fn test_new_with_empty_string_name() {
    let visitor = TaggedContentVisitor::new("", "expecting_value");
}

#[test]
fn test_new_with_empty_string_expectation() {
    let visitor = TaggedContentVisitor::new("valid_tag", "");
}

#[test]
fn test_new_with_both_empty_strings() {
    let visitor = TaggedContentVisitor::new("", "");
}

#[test]
fn test_new_with_long_static_strings() {
    let visitor = TaggedContentVisitor::new("a_long_tag_name", "a_long_expectation_value");
}

