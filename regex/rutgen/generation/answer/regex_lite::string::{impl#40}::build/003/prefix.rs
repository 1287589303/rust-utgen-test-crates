// Answer 0

#[test]
fn test_build_valid_pattern_empty() {
    let mut builder = RegexBuilder::new("");
    builder.size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_valid_pattern_single_char() {
    let mut builder = RegexBuilder::new("a");
    builder.size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_valid_pattern_wildcard() {
    let mut builder = RegexBuilder::new(".*");
    builder.size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_valid_pattern_character_class() {
    let mut builder = RegexBuilder::new("[a-z]");
    builder.size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_valid_pattern_alternation() {
    let mut builder = RegexBuilder::new("(abc|def)");
    builder.size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_valid_pattern_repetition() {
    let mut builder = RegexBuilder::new("a{1,3}");
    builder.size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_valid_pattern_case_insensitive() {
    let mut builder = RegexBuilder::new("(?i)abc");
    builder.size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_with_size_limit_none() {
    let mut builder = RegexBuilder::new("a");
    builder.size_limit(0); 
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_with_size_limit_one() {
    let mut builder = RegexBuilder::new("a");
    builder.size_limit(1);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_with_nest_limit_zero() {
    let mut builder = RegexBuilder::new("a");
    builder.nest_limit(0);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_with_case_insensitive_flag() {
    let mut builder = RegexBuilder::new("abc");
    builder.case_insensitive(true).size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_with_multi_line_flag() {
    let mut builder = RegexBuilder::new("abc");
    builder.multi_line(true).size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

#[test]
fn test_build_with_dot_matches_new_line_flag() {
    let mut builder = RegexBuilder::new("abc");
    builder.dot_matches_new_line(true).size_limit(100).nest_limit(10);
    let _regex = builder.build().unwrap();
}

