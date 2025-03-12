// Answer 0

#[test]
fn test_empty_min_range_set_to_true() {
    let mut builder = ParserBuilder::new();
    builder.empty_min_range(true);
}

#[test]
fn test_empty_min_range_set_to_false() {
    let mut builder = ParserBuilder::new();
    builder.empty_min_range(false);
}

