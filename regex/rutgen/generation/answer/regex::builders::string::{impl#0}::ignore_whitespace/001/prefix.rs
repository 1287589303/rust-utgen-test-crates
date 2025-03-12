// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)\s+(?<last>\p{Uppercase}\w*)\b";
    let result = RegexBuilder::new(pat)
        .ignore_whitespace(true)
        .build();
}

#[test]
fn test_ignore_whitespace_false() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)\s+(?<last>\p{Uppercase}\w*)\b";
    let result = RegexBuilder::new(pat)
        .ignore_whitespace(false)
        .build();
}

#[test]
fn test_ignore_whitespace_with_empty_pattern() {
    let pat = "";
    let result = RegexBuilder::new(pat)
        .ignore_whitespace(true)
        .build();
}

#[test]
fn test_ignore_whitespace_with_complex_pattern() {
    let pat = r"
        \b
        (?<first>\p{Uppercase}\w*)  # first name
        [\s--\n]+                   # separator
        (?<last>\p{Uppercase}\w*)
        \b
    ";
    let result = RegexBuilder::new(pat)
        .ignore_whitespace(true)
        .build();
}

#[test]
fn test_ignore_whitespace_with_special_characters() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)\s+[&*#]+\s+(?<last>\p{Uppercase}\w*)\b";
    let result = RegexBuilder::new(pat)
        .ignore_whitespace(true)
        .build();
}

