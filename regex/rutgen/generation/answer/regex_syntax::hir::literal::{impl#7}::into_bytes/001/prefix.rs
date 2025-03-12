// Answer 0

#[test]
fn test_into_bytes_empty() {
    let literal = Literal::exact(Vec::<u8>::new());
    let bytes = literal.into_bytes();
}

#[test]
fn test_into_bytes_single_element() {
    let literal = Literal::exact(vec![42]);
    let bytes = literal.into_bytes();
}

#[test]
fn test_into_bytes_multiple_elements() {
    let literal = Literal::exact(vec![1, 2, 3, 4, 5]);
    let bytes = literal.into_bytes();
}

#[test]
fn test_into_bytes_large_vector() {
    let large_vector: Vec<u8> = (0..255).collect();
    let literal = Literal::exact(large_vector);
    let bytes = literal.into_bytes();
}

#[test]
fn test_into_bytes_full_range() {
    let full_range: Vec<u8> = (0..=255).collect();
    let literal = Literal::exact(full_range);
    let bytes = literal.into_bytes();
}

