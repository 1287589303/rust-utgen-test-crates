// Answer 0

#[test]
fn test_eq_matching_non_empty_string() {
    let string: &str = "hello";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([104, 101, 108, 108, 111]))).unwrap(),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let result = string.eq(&bytes_mut);
}

#[test]
fn test_eq_matching_empty_string() {
    let string: &str = "";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([])).cast()).unwrap(),
        len: 0,
        cap: 0,
        data: ptr::null_mut(),
    };
    let result = string.eq(&bytes_mut);
}

#[test]
fn test_eq_non_matching_length() {
    let string: &str = "test";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([116, 101, 115, 116, 121]))).unwrap(),
        len: 5,
        cap: 5,
        data: ptr::null_mut(),
    };
    let result = string.eq(&bytes_mut);
}

#[test]
fn test_eq_non_matching_content() {
    let string: &str = "abc";
    let bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new([120, 121, 122]))).unwrap(),
        len: 3,
        cap: 3,
        data: ptr::null_mut(),
    };
    let result = string.eq(&bytes_mut);
}

