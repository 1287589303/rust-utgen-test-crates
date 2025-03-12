// Answer 0

#[test]
fn test_crlf_true() {
    let mut builder = RegexSetBuilder::new(vec![r"^foo$"]);
    builder.crlf(true);
    let _ = builder.build().unwrap();
}

#[test]
fn test_crlf_false() {
    let mut builder = RegexSetBuilder::new(vec![r"^foo$"]);
    builder.crlf(false);
    let _ = builder.build().unwrap();
}

