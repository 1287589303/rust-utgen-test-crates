// Answer 0

#[test]
fn test_crlf_true() {
    let mut builder = RegexBuilder::new(r"^foo$");
    let result = builder.multi_line(true).crlf(true);
    let _ = result.build().unwrap();
}

#[test]
fn test_crlf_false() {
    let mut builder = RegexBuilder::new(r"^foo$");
    let result = builder.multi_line(true).crlf(false);
    let _ = result.build().unwrap();
}

