// Answer 0

#[test]
fn test_new_valid_name() {
    let visitor = TagOrContentVisitor::new("test");
}

#[test]
fn test_new_empty_name() {
    let visitor = TagOrContentVisitor::new("");
}

#[test]
fn test_new_single_character_name() {
    let visitor = TagOrContentVisitor::new("a");
}

#[test]
fn test_new_long_name() {
    let visitor = TagOrContentVisitor::new("example");
}

