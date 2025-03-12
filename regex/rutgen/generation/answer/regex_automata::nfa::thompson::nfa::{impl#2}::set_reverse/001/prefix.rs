// Answer 0

#[test]
fn test_set_reverse_true() {
    let mut inner = Inner::default();
    inner.set_reverse(true);
}

#[test]
fn test_set_reverse_false() {
    let mut inner = Inner::default();
    inner.set_reverse(false);
}

