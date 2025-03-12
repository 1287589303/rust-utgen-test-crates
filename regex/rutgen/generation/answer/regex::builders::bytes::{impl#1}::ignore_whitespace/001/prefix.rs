// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let pat = r"\b(?<name>\w+)\b";
    let result = RegexSetBuilder::new([pat])
        .ignore_whitespace(true)
        .build()
        .unwrap();
    let _ = result.is_match(b"Harry Potter");
}

#[test]
fn test_ignore_whitespace_false() {
    let pat = r"\b(?<name>\w+)\b";
    let result = RegexSetBuilder::new([pat])
        .ignore_whitespace(false)
        .build()
        .unwrap();
    let _ = result.is_match(b"Harry Potter");
}

#[test]
fn test_empty_patterns() {
    let result = RegexSetBuilder::new::<Vec<&str>, &str>(Vec::<&str>::new())
        .ignore_whitespace(true)
        .build();
}

#[test]
fn test_size_limit() {
    let pat = r"\b(?<name>\w+)\b";
    let result = RegexSetBuilder::new([pat])
        .size_limit(100)
        .build()
        .unwrap();
    let _ = result.is_match(b"Harry Potter");
}

#[test]
fn test_dfa_size_limit() {
    let pat = r"\b(?<name>\w+)\b";
    let result = RegexSetBuilder::new([pat])
        .dfa_size_limit(200)
        .build()
        .unwrap();
    let _ = result.is_match(b"Harry Potter");
}

#[test]
fn test_nest_limit() {
    let pat = r"\b(?<name>\w+)\b";
    let result = RegexSetBuilder::new([pat])
        .nest_limit(10)
        .build()
        .unwrap();
    let _ = result.is_match(b"Harry Potter");
}

#[test]
fn test_line_terminator() {
    let pat = r"\b(?<name>\w+)\b";
    let result = RegexSetBuilder::new([pat])
        .line_terminator(10)
        .build()
        .unwrap();
    let _ = result.is_match(b"Harry Potter");
}

