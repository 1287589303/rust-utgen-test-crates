// Answer 0

#[test]
fn test_build_valid_pattern_simple() {
    let builder = Builder::new();
    let pattern = "a*b";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_valid_pattern_anchored() {
    let builder = Builder::new();
    let pattern = "^[a-z]+$";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_valid_pattern_alternation() {
    let builder = Builder::new();
    let pattern = "(\\d+)|([a-z]+)";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_invalid_pattern_unmatched_paren() {
    let builder = Builder::new();
    let pattern = "(a";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_invalid_pattern_empty_class() {
    let builder = Builder::new();
    let pattern = "[]";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_invalid_pattern_incomplete_class() {
    let builder = Builder::new();
    let pattern = "[a-z";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_valid_pattern_escape_sequences() {
    let builder = Builder::new();
    let pattern = "a\\*b";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_valid_pattern_unicode() {
    let builder = Builder::new();
    let pattern = "你好";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_empty_pattern() {
    let builder = Builder::new();
    let pattern = "";
    let _ = builder.build(pattern);
}

#[test]
fn test_build_long_valid_pattern() {
    let builder = Builder::new();
    let pattern = "a".repeat(100);
    let _ = builder.build(&pattern);
}

