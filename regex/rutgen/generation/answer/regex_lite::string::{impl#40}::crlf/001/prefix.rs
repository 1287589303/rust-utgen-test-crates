// Answer 0

#[test]
fn test_crlf_enabled() {
    let mut builder = RegexBuilder::new(r"^foo$");
    let re = builder.multi_line(true).crlf(true).build().unwrap();
    let hay = "\r\nfoo\r\n";
    re.find(hay);
}

#[test]
fn test_crlf_disabled() {
    let mut builder = RegexBuilder::new(r"^foo$");
    let re = builder.multi_line(true).crlf(false).build().unwrap();
    let hay = "\r\nfoo\r\n";
    re.find(hay);
}

#[test]
fn test_crlf_with_empty_pattern() {
    let mut builder = RegexBuilder::new(r"");
    let re = builder.multi_line(true).crlf(true).build().unwrap();
    let hay = "\r\n\r\n";
    re.find(hay);
}

#[test]
fn test_crlf_on_pattern_start() {
    let mut builder = RegexBuilder::new(r"^");
    let re = builder.multi_line(true).crlf(true).build().unwrap();
    let hay = "\r\nabc";
    re.find(hay);
}

#[test]
fn test_crlf_on_pattern_end() {
    let mut builder = RegexBuilder::new(r"$");
    let re = builder.multi_line(true).crlf(true).build().unwrap();
    let hay = "abc\r\n";
    re.find(hay);
}

