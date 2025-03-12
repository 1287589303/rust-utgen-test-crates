// Answer 0

#[test]
#[should_panic]
fn test_set_len_exceeds_capacity() {
    let mut buffer = unsafe {
        let mut b = BytesMut::with_capacity(10);
        b.set_len(11); // This should panic as 11 > 10
        b
    };
    unsafe { buffer.set_len(15); } // This should panic as well
}

#[test]
#[should_panic]
fn test_set_len_max_value() {
    let mut buffer = unsafe {
        let mut b = BytesMut::with_capacity(1);
        b.set_len(usize::MAX); // This should panic as usize::MAX > 1
        b
    };
}

#[test]
#[should_panic]
fn test_set_len_large_value() {
    let mut buffer = unsafe {
        let mut b = BytesMut::with_capacity(100);
        b.set_len(101); // This should panic as 101 > 100
        b
    };
}

