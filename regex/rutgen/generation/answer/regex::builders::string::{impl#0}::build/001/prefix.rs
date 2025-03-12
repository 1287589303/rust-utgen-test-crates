// Answer 0

#[test]
fn test_build_valid_regex() {
    let mut builder = RegexBuilder::new("^[a-zA-Z0-9]*$");
    let result = builder.build();
}

#[test]
fn test_build_empty_regex() {
    let mut builder = RegexBuilder::new("");
    let result = builder.build();
}

#[test]
fn test_build_simple_regex_case_insensitive() {
    let mut builder = RegexBuilder::new("abc");
    builder.case_insensitive(true);
    let result = builder.build();
}

#[test]
fn test_build_complex_regex_multi_line() {
    let mut builder = RegexBuilder::new("(?m)^abc");
    builder.multi_line(true);
    let result = builder.build();
}

#[test]
fn test_build_regex_with_dot_matches_new_line() {
    let mut builder = RegexBuilder::new("a.b");
    builder.dot_matches_new_line(true);
    let result = builder.build();
}

#[test]
fn test_build_regex_with_crlf() {
    let mut builder = RegexBuilder::new("abc\r\nxyz");
    builder.crlf(true);
    let result = builder.build();
}

#[test]
fn test_build_regex_with_swap_greed() {
    let mut builder = RegexBuilder::new("(.*)");
    builder.swap_greed(true);
    let result = builder.build();
}

#[test]
fn test_build_regex_with_ignore_whitespace() {
    let mut builder = RegexBuilder::new("a # this is a comment\nb");
    builder.ignore_whitespace(true);
    let result = builder.build();
}

#[test]
fn test_build_regex_with_octal() {
    let mut builder = RegexBuilder::new("\x41\x42\x43"); // Matches "ABC"
    builder.octal(true);
    let result = builder.build();
}

#[test]
fn test_build_with_size_limit_zero() {
    let mut builder = RegexBuilder::new("abc");
    builder.size_limit(0);
    let result = builder.build();
}

#[test]
fn test_build_with_size_limit_max() {
    let mut builder = RegexBuilder::new("a".repeat(1_000_000)); // Large regex
    builder.size_limit(1_000_000);
    let result = builder.build();
}

#[test]
fn test_build_large_regex_exceeding_size_limit() {
    let mut builder = RegexBuilder::new("a".repeat(1_000_001)); // Exceeds size limit
    builder.size_limit(1_000_000);
    let result = builder.build();
}

