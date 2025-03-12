// Answer 0

#[test]
fn test_set_utf8_true() {
    let mut inner = Inner::default();
    inner.set_utf8(true);
}

#[test]
fn test_set_utf8_false() {
    let mut inner = Inner::default();
    inner.set_utf8(false);
}

