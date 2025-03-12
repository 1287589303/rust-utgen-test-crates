// Answer 0

#[test]
fn test_put_bytes_with_exact_fit() {
    let mut buf = [0; 4];
    {
        let mut slice = &mut buf[..];
        slice.put_bytes(b'a', 4);
    }
}

#[test]
fn test_put_bytes_with_one_fit() {
    let mut buf = [0; 1];
    {
        let mut slice = &mut buf[..];
        slice.put_bytes(b'a', 1);
    }
}

#[test]
fn test_put_bytes_with_two_fit() {
    let mut buf = [0; 2];
    {
        let mut slice = &mut buf[..];
        slice.put_bytes(b'a', 2);
    }
}

#[test]
fn test_put_bytes_with_three_fit() {
    let mut buf = [0; 3];
    {
        let mut slice = &mut buf[..];
        slice.put_bytes(b'a', 3);
    }
}

#[test]
fn test_put_bytes_with_zero_cnt() {
    let mut buf = [0; 6];
    {
        let mut slice = &mut buf[..];
        slice.put_bytes(b'a', 0);
    }
}

#[should_panic]
#[test]
fn test_put_bytes_with_insufficient_capacity() {
    let mut buf = [0; 0]; // remaining_mut() = 0
    {
        let mut slice = &mut buf[..];
        slice.put_bytes(b'a', 1); // panic case, cnt = 1
    }
}

