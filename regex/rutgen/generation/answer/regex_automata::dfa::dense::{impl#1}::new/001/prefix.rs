// Answer 0

#[test]
fn test_builder_new_default() {
    let builder = Builder::new();
}

#[cfg(feature = "syntax")]
#[test]
fn test_builder_new_with_syntax() {
    let builder = Builder::new();
    let thompson_compiler = builder.thompson.clone(); 
}

