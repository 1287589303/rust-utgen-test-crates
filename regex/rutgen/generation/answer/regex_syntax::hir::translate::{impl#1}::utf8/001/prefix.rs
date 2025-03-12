// Answer 0

#[test]
fn test_utf8_enable() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.utf8(true);
}

#[test]
fn test_utf8_disable() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.utf8(false);
}

