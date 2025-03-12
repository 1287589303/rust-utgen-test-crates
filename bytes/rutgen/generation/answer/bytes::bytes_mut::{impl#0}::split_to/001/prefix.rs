// Answer 0

#[test]
fn test_split_to_empty() {
    let mut bytes_mut = BytesMut::new();
    let result = bytes_mut.split_to(0);
}

#[test]
fn test_split_to_single_element() {
    let mut bytes_mut = BytesMut::from_vec(vec![1]);
    let result = bytes_mut.split_to(1);
}

#[test]
fn test_split_to_multiple_elements() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3, 4, 5]);
    let result = bytes_mut.split_to(3);
}

#[test]
#[should_panic]
fn test_split_to_out_of_bounds() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3]);
    let result = bytes_mut.split_to(4); // Should panic
}

#[test]
fn test_split_to_boundary() {
    let mut bytes_mut = BytesMut::from_vec(vec![1, 2, 3, 4]);
    let result = bytes_mut.split_to(4); // Edge case where at == len
}

