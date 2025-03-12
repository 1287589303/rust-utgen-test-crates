// Answer 0

#[test]
fn test_builder_new_default() {
    let builder = Builder::new();
}

#[test]
#[cfg(feature = "syntax")]
fn test_builder_new_with_syntax() {
    let builder = Builder::new();
}

