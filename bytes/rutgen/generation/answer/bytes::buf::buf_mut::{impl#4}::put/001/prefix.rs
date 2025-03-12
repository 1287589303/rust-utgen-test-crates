// Answer 0

#[test]
fn test_put_with_remaining_data() {
    let mut vec = Vec::with_capacity(10);
    let src = vec![1, 2, 3, 4];
    unsafe {
        vec.put(src);
    }
}

#[test]
fn test_put_with_no_remaining_data() {
    let mut vec = Vec::with_capacity(10);
    let src: Vec<u8> = Vec::new();
    unsafe {
        vec.put(src);
    }
}

