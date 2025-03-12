// Answer 0

#[test]
#[should_panic]
fn test_split_off_negative_index() {
    let mut bytes = BytesMut::with_capacity(10);
    let _ = bytes.split_off(11); // at > self.capacity()
}

#[test]
#[should_panic]
fn test_split_off_zero_capacity() {
    let mut bytes = BytesMut::new();
    let _ = bytes.split_off(1); // at > self.capacity()
}

#[test]
#[should_panic]
fn test_split_off_large_index() {
    let mut bytes = BytesMut::with_capacity(5);
    let _ = bytes.split_off(10); // at > self.capacity()
}

