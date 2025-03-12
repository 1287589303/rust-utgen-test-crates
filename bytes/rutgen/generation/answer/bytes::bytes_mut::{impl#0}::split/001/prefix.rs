// Answer 0

#[test]
fn test_split_empty() {
    let mut buf = BytesMut::new();
    let other = buf.split();
}

#[test]
fn test_split_single_byte() {
    let mut buf = BytesMut::with_capacity(1);
    buf.resize(1, 0u8);
    let other = buf.split();
}

#[test]
fn test_split_large_capacity() {
    let mut buf = BytesMut::with_capacity(1024);
    let data = b"hello world";
    buf.extend_from_slice(data);
    let other = buf.split();
}

