// Answer 0

#[test]
fn test_case_insensitive_true() {
    let re = RegexBuilder::new("foo(?-i:bar)quux")
        .case_insensitive(true)
        .build()
        .unwrap();
    re.is_match(b"FoObarQuUx");
}

#[test]
fn test_case_insensitive_false() {
    let re = RegexBuilder::new("foo(?-i:bar)quux")
        .case_insensitive(false)
        .build()
        .unwrap();
    re.is_match(b"fooBARquux");
}

#[test]
fn test_case_insensitive_with_empty_string() {
    let re = RegexBuilder::new("foo")
        .case_insensitive(true)
        .build()
        .unwrap();
    re.is_match(b"FOO");
}

#[test]
fn test_case_insensitive_on_pattern_only() {
    let re = RegexBuilder::new("(?i:foo)bar")
        .case_insensitive(true)
        .build()
        .unwrap();
    re.is_match(b"FOObar");
}

#[test]
fn test_case_insensitive_with_size_limit() {
    let re = RegexBuilder::new("foo")
        .size_limit(100)
        .case_insensitive(true)
        .build()
        .unwrap();
    re.is_match(b"FOO");
}

#[test]
fn test_case_insensitive_with_dfa_size_limit() {
    let re = RegexBuilder::new("foo")
        .dfa_size_limit(100)
        .case_insensitive(false)
        .build()
        .unwrap();
    re.is_match(b"foo");
}

#[test]
fn test_case_insensitive_with_line_terminator() {
    let re = RegexBuilder::new("foo")
        .line_terminator(10)
        .case_insensitive(true)
        .build()
        .unwrap();
    re.is_match(b"FOO");
}

#[test]
fn test_case_insensitive_with_nest_limit() {
    let re = RegexBuilder::new("(foo(bar)?)")
        .nest_limit(2)
        .case_insensitive(true)
        .build()
        .unwrap();
    re.is_match(b"FOOBAR");
}

