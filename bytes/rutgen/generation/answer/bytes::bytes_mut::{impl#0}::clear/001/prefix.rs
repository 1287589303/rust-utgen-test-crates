// Answer 0

#[test]
fn test_clear_nonempty_buffer() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(&[1, 2, 3]);
    buf.clear();
    buf.len();
}

#[test]
fn test_clear_empty_buffer() {
    let mut buf = BytesMut::new();
    buf.clear();
    buf.len();
}

#[test]
fn test_clear_buffer_after_resize() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(5, 0);
    buf.clear();
    buf.len();
}

#[test]
fn test_clear_after_split() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    let _other = buf.split_off(3);
    buf.clear();
    buf.len();
}

