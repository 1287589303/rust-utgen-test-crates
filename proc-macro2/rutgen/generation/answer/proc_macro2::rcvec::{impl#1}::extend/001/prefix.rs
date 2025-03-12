// Answer 0

#[test]
fn test_extend_empty_iterator() {
    let mut builder = RcVecBuilder::new();
    let iter: Vec<i32> = vec![];
    builder.extend(iter);
}

#[test]
fn test_extend_single_element_iterator() {
    let mut builder = RcVecBuilder::new();
    let iter = vec![42];
    builder.extend(iter);
}

#[test]
fn test_extend_multi_element_iterator() {
    let mut builder = RcVecBuilder::new();
    let iter = vec![1, 2, 3, 4, 5];
    builder.extend(iter);
}

#[test]
fn test_extend_iterator_with_greater_capacity() {
    let mut builder = RcVecBuilder::with_capacity(3);
    let iter = vec![6, 7, 8, 9, 10];
    builder.extend(iter);
}

#[test]
fn test_extend_iterator_with_mixed_types() {
    struct MixedType;
    let mut builder = RcVecBuilder::new();
    let iter: Vec<MixedType> = vec![MixedType, MixedType];
    builder.extend(iter);
}

#[test]
fn test_extend_boundary_capacity() {
    let capacity = core::usize::MAX;
    let mut builder = RcVecBuilder::with_capacity(capacity);
    let iter = vec![1; capacity];
    builder.extend(iter);
}

