// Answer 0

#[test]
fn test_from_iter_empty() {
    let iter: Vec<u8> = Vec::new();
    let bytes_mut = BytesMut::from_iter(iter);
}

#[test]
fn test_from_iter_one_element() {
    let iter: Vec<u8> = vec![42];
    let bytes_mut = BytesMut::from_iter(iter);
}

#[test]
fn test_from_iter_max_length() {
    let iter: Vec<u8> = (0..256).collect();
    let bytes_mut = BytesMut::from_iter(iter);
}

#[test]
fn test_from_iter_exceeding_capacity() {
    let iter: Vec<u8> = (0..260).collect(); // boundary case
    let bytes_mut = BytesMut::from_iter(iter);
}

#[test]
fn test_from_iter_repeated_elements() {
    let iter: Vec<u8> = vec![255; 10]; // 10 elements
    let bytes_mut = BytesMut::from_iter(iter);
}

#[test]
fn test_from_iter_large_capacity() {
    let iter: Vec<u8> = (0..256).collect();
    let bytes_mut = BytesMut::from_iter(iter);
    let capacity = bytes_mut.capacity();
    assert!(capacity >= 256);
}

#[test]
fn test_from_iter_boundary_capacity() {
    let iter: Vec<u8> = (0..255).collect();
    let bytes_mut = BytesMut::from_iter(iter);
    let capacity = bytes_mut.capacity();
    assert!(capacity >= 255);
}

