// Answer 0

#[test]
fn test_as_ref_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(5);
    }
    let slice: &[u8] = bytes_mut.as_ref();
}

#[test]
fn test_as_ref_empty() {
    let bytes_mut = BytesMut::new();
    let slice: &[u8] = bytes_mut.as_ref();
}

#[test]
fn test_as_ref_with_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe {
        bytes_mut.set_len(15);
    }
    let slice: &[u8] = bytes_mut.as_ref();
}

#[test]
fn test_as_ref_zeroed() {
    let mut bytes_mut = BytesMut::zeroed(10);
    let slice: &[u8] = bytes_mut.as_ref();
}

#[test]
fn test_as_ref_split() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(5);
    }
    let other = bytes_mut.split();
    let slice: &[u8] = other.as_ref();
}

