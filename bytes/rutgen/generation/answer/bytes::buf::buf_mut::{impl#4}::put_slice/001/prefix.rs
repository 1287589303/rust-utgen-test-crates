// Answer 0

#[test]
fn test_put_slice_empty() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    let src: &[u8] = &[];
    buffer.put_slice(src);
}

#[test]
fn test_put_slice_partial_fill() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    let src: &[u8] = &[1, 2, 3, 4];
    buffer.extend_from_slice(&[0; 4]); // Ensure buffer has remaining capacity
    buffer.put_slice(src);
}

#[test]
fn test_put_slice_full_fill() {
    let mut buffer: Vec<u8> = Vec::with_capacity(4);
    let src: &[u8] = &[1, 2, 3, 4];
    buffer.put_slice(src);
}

#[test]
#[should_panic]
fn test_put_slice_overfill() {
    let mut buffer: Vec<u8> = Vec::with_capacity(4);
    let src: &[u8] = &[1, 2, 3, 4, 5]; // Attempt to overfill the buffer
    buffer.put_slice(src);
}

