// Answer 0

#[test]
fn test_crlf_enabled() {
    let mut builder = RegexSetBuilder::new([r"^foo$"]);
    builder.multi_line(true).crlf(true);
    let regex_set = builder.build().unwrap();
    let hay = "\r\nfoo\r\n";
    let _ = regex_set.is_match(hay);
}

#[test]
fn test_crlf_disabled() {
    let mut builder = RegexSetBuilder::new([r"^foo$"]);
    builder.multi_line(true).crlf(false);
    let regex_set = builder.build().unwrap();
    let hay = "\r\nfoo\r\n";
    let _ = regex_set.is_match(hay);
}

#[test]
fn test_crlf_with_empty_pattern() {
    let mut builder = RegexSetBuilder::new([r""]);
    builder.multi_line(true).crlf(true);
    let regex_set = builder.build().unwrap();
    let hay = "\r\n";
    let _ = regex_set.is_match(hay);
}

#[test]
fn test_crlf_with_newline_only() {
    let mut builder = RegexSetBuilder::new([r"^\n"]);
    builder.multi_line(true).crlf(true);
    let regex_set = builder.build().unwrap();
    let _ = regex_set.is_match("\r\n");
}

