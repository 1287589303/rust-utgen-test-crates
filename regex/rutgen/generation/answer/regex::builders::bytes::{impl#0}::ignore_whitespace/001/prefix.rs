// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    let re = RegexBuilder::new(pat)
        .ignore_whitespace(true)
        .build()
        .unwrap();
}

#[test]
fn test_ignore_whitespace_false() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    let re = RegexBuilder::new(pat)
        .ignore_whitespace(false)
        .build()
        .unwrap();
}

#[test]
fn test_line_terminator() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    for byte in 0..=255 {
        let re = RegexBuilder::new(pat)
            .line_terminator(byte)
            .build()
            .unwrap();
    }
}

#[test]
fn test_size_limit() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    for limit in [0, 1, 10_000, 10_000_000].iter() {
        let re = RegexBuilder::new(pat)
            .size_limit(*limit)
            .build()
            .unwrap();
    }
}

#[test]
fn test_dfa_size_limit() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    for limit in [0, 1, 10_000, 10_000_000].iter() {
        let re = RegexBuilder::new(pat)
            .dfa_size_limit(*limit)
            .build()
            .unwrap();
    }
}

#[test]
fn test_nest_limit() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    for limit in 1..=100 {
        let re = RegexBuilder::new(pat)
            .nest_limit(limit)
            .build()
            .unwrap();
    }
}

#[test]
fn test_unicode_enabled() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    let re = RegexBuilder::new(pat)
        .unicode(true)
        .build()
        .unwrap();
}

#[test]
fn test_unicode_disabled() {
    let pat = r"\b(?<first>\p{Uppercase}\w*)[\s--\n]+(?<last>\p{Uppercase}\w*)\b";
    let re = RegexBuilder::new(pat)
        .unicode(false)
        .build()
        .unwrap();
}

