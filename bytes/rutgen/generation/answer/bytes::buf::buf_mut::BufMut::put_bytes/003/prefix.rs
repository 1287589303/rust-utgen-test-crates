// Answer 0

#[test]
fn test_put_bytes_exact_capacity() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 6);
    }
}

#[test]
#[should_panic]
fn test_put_bytes_exceeding_capacity() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 7);
    }
}

#[test]
fn test_put_bytes_zero_count() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 0);
    }
}

