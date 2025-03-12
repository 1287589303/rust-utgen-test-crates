// Answer 0

#[test]
#[cfg(feature = "syntax")]
fn test_builder_new_with_syntax_enabled() {
    let builder = Builder::new();
}

#[test]
#[cfg(not(feature = "syntax"))]
fn test_builder_new_with_syntax_disabled() {
    let builder = Builder::new();
}

