// Answer 0

#[test]
fn test_get_reverse_default() {
    let builder = Builder::new();
    let result = builder.get_reverse();
}

#[test]
fn test_get_reverse_true() {
    let mut builder = Builder::new();
    builder.set_reverse(true);
    let result = builder.get_reverse();
}

#[test]
fn test_get_reverse_false() {
    let mut builder = Builder::new();
    builder.set_reverse(false);
    let result = builder.get_reverse();
}

