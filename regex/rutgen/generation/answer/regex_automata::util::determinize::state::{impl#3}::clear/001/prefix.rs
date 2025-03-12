// Answer 0

#[test]
fn test_clear_non_empty_vec() {
    let mut builder = StateBuilderEmpty(vec![1, 2, 3]);
    builder.clear();
}

#[test]
fn test_clear_with_capacity() {
    let mut builder = StateBuilderEmpty(vec![0; 10]);
    builder.clear();
}

#[test]
fn test_clear_singular_element() {
    let mut builder = StateBuilderEmpty(vec![42]);
    builder.clear();
}

