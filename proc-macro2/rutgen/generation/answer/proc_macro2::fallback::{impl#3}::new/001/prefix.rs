// Answer 0

#[test]
fn test_token_stream_builder_new() {
    let tsb = TokenStreamBuilder::new();
}

#[test]
fn test_token_stream_builder_with_capacity_zero() {
    let tsb = TokenStreamBuilder::with_capacity(0);
}

#[test]
fn test_token_stream_builder_with_capacity_ten() {
    let tsb = TokenStreamBuilder::with_capacity(10);
}

