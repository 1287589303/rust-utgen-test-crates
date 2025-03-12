// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)";
    let mut builder = RegexSetBuilder::new(vec![pat]);
    let result = builder.ignore_whitespace(true).build();
}

#[test]
fn test_ignore_whitespace_false() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)";
    let mut builder = RegexSetBuilder::new(vec![pat]);
    let result = builder.ignore_whitespace(false).build();
}

#[test]
fn test_ignore_whitespace_multiple_patterns() {
    let pats = vec![
        r"\b(?<first>\p{Uppercase}\w*)",
        r"\b(?<last>\p{Uppercase}\w*)"
    ];
    let mut builder = RegexSetBuilder::new(pats);
    let result = builder.ignore_whitespace(true).build();
}

#[test]
fn test_ignore_whitespace_empty_pattern() {
    let pats: Vec<&str> = vec![];
    let mut builder = RegexSetBuilder::new(pats);
    let result = builder.ignore_whitespace(true).build();
}

#[test]
fn test_ignore_whitespace_large_pattern() {
    let pats = vec![r"(?x)    # Example pattern with comments
                    \b(?<first>\p{Uppercase}\w*)
                    [\s--\n]+"];
    let mut builder = RegexSetBuilder::new(pats);
    let result = builder.ignore_whitespace(true).build();
}

