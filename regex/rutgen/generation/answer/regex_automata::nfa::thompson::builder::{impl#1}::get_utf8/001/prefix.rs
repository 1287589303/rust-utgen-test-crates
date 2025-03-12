// Answer 0

#[test]
fn test_get_utf8_true() {
    let mut builder = Builder::default();
    builder.set_utf8(true);
    let result = builder.get_utf8();
}

#[test]
fn test_get_utf8_false() {
    let mut builder = Builder::default();
    builder.set_utf8(false);
    let result = builder.get_utf8();
}

