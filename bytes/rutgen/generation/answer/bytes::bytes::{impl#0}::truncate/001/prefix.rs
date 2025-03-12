// Answer 0

#[test]
fn test_truncate_promotable_even_vtable_len_1() {
    let mut buf = {
        let data = b"abcd";
        let bytes = Bytes::from_static(data);
        bytes.clone()
    };
    buf.truncate(1);
}

#[test]
fn test_truncate_promotable_even_vtable_len_2() {
    let mut buf = {
        let data = b"abcd";
        let bytes = Bytes::from_static(data);
        bytes.clone()
    };
    buf.truncate(2);
}

#[test]
fn test_truncate_promotable_even_vtable_len_3() {
    let mut buf = {
        let data = b"abcd";
        let bytes = Bytes::from_static(data);
        bytes.clone()
    };
    buf.truncate(3);
}

#[test]
fn test_truncate_promotable_even_vtable_len_4() {
    let mut buf = {
        let data = b"abcdef";
        let bytes = Bytes::from_static(data);
        bytes.clone()
    };
    buf.truncate(4);
}

