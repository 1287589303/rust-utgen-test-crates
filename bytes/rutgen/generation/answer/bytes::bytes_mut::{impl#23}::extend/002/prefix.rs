// Answer 0

#[test]
fn test_extend_empty_iterator() {
    let mut bytes_mut = BytesMut::new();
    let empty_iter: Vec<u8> = vec![];
    bytes_mut.extend(empty_iter);
}

#[test]
fn test_extend_single_element() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let single_element = vec![42]; // Testing with a single u8 element
    bytes_mut.extend(single_element);
}

#[test]
fn test_extend_multiple_elements() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let multiple_elements = vec![10, 20, 30, 40, 50]; // Multiple u8 elements
    bytes_mut.extend(multiple_elements);
}

#[test]
fn test_extend_max_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let max_elements = vec![255; 10]; // Filling to capacity with 255
    bytes_mut.extend(max_elements);
}

#[test]
fn test_extend_exceed_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    let exceed_elements = vec![1, 2, 3, 4, 5, 6]; // Exceeding the initial capacity
    bytes_mut.extend(exceed_elements);
}

