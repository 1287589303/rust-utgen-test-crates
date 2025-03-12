// Answer 0

#[test]
fn test_token_stream_builder_with_capacity_zero() {
    let cap: usize = 0;
    let _builder = TokenStreamBuilder::with_capacity(cap);
}

#[test]
fn test_token_stream_builder_with_capacity_one() {
    let cap: usize = 1;
    let _builder = TokenStreamBuilder::with_capacity(cap);
}

#[test]
fn test_token_stream_builder_with_capacity_ten() {
    let cap: usize = 10;
    let _builder = TokenStreamBuilder::with_capacity(cap);
}

#[test]
fn test_token_stream_builder_with_capacity_max() {
    let cap: usize = usize::MAX;
    let _builder = TokenStreamBuilder::with_capacity(cap);
}

