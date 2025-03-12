// Answer 0

#[test]
fn test_get_ref_with_u8() {
    struct TestBuf {
        data: Vec<u8>,
    }

    let data = vec![b'a', b'b', b'c'];
    let iter = IntoIter::new(TestBuf { data });
    let _ref = iter.get_ref();
}

#[test]
fn test_get_ref_with_str() {
    struct TestBuf {
        data: String,
    }

    let data = String::from("abc");
    let iter = IntoIter::new(TestBuf { data });
    let _ref = iter.get_ref();
}

#[test]
fn test_get_ref_with_vec() {
    struct TestBuf {
        data: Vec<u8>,
    }

    let data = vec![1, 2, 3];
    let iter = IntoIter::new(TestBuf { data });
    let _ref = iter.get_ref();
}

#[test]
fn test_get_ref_with_empty_vec() {
    struct TestBuf {
        data: Vec<u8>,
    }

    let data = vec![];
    let iter = IntoIter::new(TestBuf { data });
    let _ref = iter.get_ref();
}

#[test]
fn test_get_ref_with_bytes() {
    struct TestBuf {
        data: bytes::Bytes,
    }

    let data = bytes::Bytes::from(&b"abc"[..]);
    let iter = IntoIter::new(TestBuf { data });
    let _ref = iter.get_ref();
}

