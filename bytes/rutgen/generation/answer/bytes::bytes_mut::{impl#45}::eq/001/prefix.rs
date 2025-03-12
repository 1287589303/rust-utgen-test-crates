// Answer 0

#[test]
fn test_eq_non_empty_string_equal_bytes_mut() {
    let non_empty_string = String::from("test");
    let other = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(non_empty_string.clone()))).unwrap(),
        len: non_empty_string.len(),
        cap: non_empty_string.len(),
        data: ptr::null_mut(),
    };
    let _ = non_empty_string.eq(&other);
}

#[test]
fn test_eq_non_empty_string_not_equal_bytes_mut() {
    let non_empty_string = String::from("test");
    let other = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(String::from("different")))).unwrap(),
        len: "different".len(),
        cap: "different".len(),
        data: ptr::null_mut(),
    };
    let _ = non_empty_string.eq(&other);
}

#[test]
fn test_eq_empty_string_equal_bytes_mut() {
    let empty_string = String::from("");
    let other = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(empty_string.clone()))).unwrap(),
        len: empty_string.len(),
        cap: empty_string.len(),
        data: ptr::null_mut(),
    };
    let _ = empty_string.eq(&other);
}

#[test]
fn test_eq_empty_string_not_equal_bytes_mut() {
    let empty_string = String::from("");
    let other = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(String::from("not empty")))).unwrap(),
        len: "not empty".len(),
        cap: "not empty".len(),
        data: ptr::null_mut(),
    };
    let _ = empty_string.eq(&other);
}

