// Answer 0

#[test]
fn test_partial_cmp_empty_vs_non_empty() {
    let empty = BytesMut::new();
    let non_empty = BytesMut::with_capacity(1);
    non_empty.extend_from_slice(&[1]);
    let _ = empty.partial_cmp(&non_empty);
}

#[test]
fn test_partial_cmp_non_empty_vs_empty() {
    let non_empty = BytesMut::with_capacity(1);
    non_empty.extend_from_slice(&[1]);
    let empty = BytesMut::new();
    let _ = non_empty.partial_cmp(&empty);
}

#[test]
fn test_partial_cmp_equal_length_all_zeros() {
    let buffer1 = BytesMut::with_capacity(10);
    let buffer2 = BytesMut::with_capacity(10);
    let _ = buffer1.partial_cmp(&buffer2);
}

#[test]
fn test_partial_cmp_equal_length_all_ones() {
    let mut buffer1 = BytesMut::with_capacity(10);
    buffer1.extend_from_slice(&[1; 10]);
    let mut buffer2 = BytesMut::with_capacity(10);
    buffer2.extend_from_slice(&[1; 10]);
    let _ = buffer1.partial_cmp(&buffer2);
}

#[test]
fn test_partial_cmp_different_content() {
    let mut buffer1 = BytesMut::with_capacity(10);
    buffer1.extend_from_slice(&[1, 2, 3, 4, 5]);
    let mut buffer2 = BytesMut::with_capacity(10);
    buffer2.extend_from_slice(&[5, 4, 3, 2, 1]);
    let _ = buffer1.partial_cmp(&buffer2);
}

#[test]
fn test_partial_cmp_varying_lengths() {
    let mut buffer1 = BytesMut::with_capacity(10);
    buffer1.extend_from_slice(&[0; 10]);
    let mut buffer2 = BytesMut::with_capacity(5);
    buffer2.extend_from_slice(&[0; 5]);
    let _ = buffer1.partial_cmp(&buffer2);
}

#[test]
fn test_partial_cmp_max_length() {
    let max_capacity = usize::MAX >> 5; // Account for VEC_POS_OFFSET
    let mut buffer1 = BytesMut::with_capacity(max_capacity);
    let mut buffer2 = BytesMut::with_capacity(max_capacity);
    let _ = buffer1.partial_cmp(&buffer2);
}

#[test]
fn test_partial_cmp_all_values() {
    let mut buffer1 = BytesMut::with_capacity(10);
    buffer1.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut buffer2 = BytesMut::with_capacity(10);
    buffer2.extend_from_slice(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let _ = buffer1.partial_cmp(&buffer2);
}

