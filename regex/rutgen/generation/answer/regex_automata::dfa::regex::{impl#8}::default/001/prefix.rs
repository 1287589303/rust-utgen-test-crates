// Answer 0

#[test]
fn test_builder_default() {
    let builder = Builder::default();
    let _ = builder; // Invoke to use builder and avoid unused variable warning
}

#[test]
fn test_builder_new() {
    let builder = Builder::new();
    let _ = builder; // Invoke to use builder and avoid unused variable warning
}

