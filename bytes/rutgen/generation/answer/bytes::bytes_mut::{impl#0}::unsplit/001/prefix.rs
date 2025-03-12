// Answer 0

#[test]
fn test_unsplit_empty_self_with_non_empty_other() {
    let mut self_buf = BytesMut::new();
    let other_buf = BytesMut::from_vec(vec![1, 2, 3]);
    self_buf.unsplit(other_buf);
}

#[test]
fn test_unsplit_empty_self_with_empty_other() {
    let mut self_buf = BytesMut::new();
    let other_buf = BytesMut::new();
    self_buf.unsplit(other_buf);
}

#[test]
fn test_unsplit_empty_self_with_large_other() {
    let mut self_buf = BytesMut::new();
    let other_buf = BytesMut::with_capacity(128);
    self_buf.unsplit(other_buf);
}

