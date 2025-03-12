// Answer 0

#[test]
fn test_crlf_enable() {
    let mut builder = TranslatorBuilder::new();
    builder.crlf(true);
}

#[test]
fn test_crlf_disable() {
    let mut builder = TranslatorBuilder::new();
    builder.crlf(false);
}

