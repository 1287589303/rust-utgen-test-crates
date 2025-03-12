// Answer 0

#[test]
fn test_from_vec_with_len_less_than_cap() {
    let vec: Vec<u8> = vec![1, 2, 3, 4, 5]; // len = 5, cap = default capacity
    let bytes = Bytes::from(vec);
}

#[test]
#[should_panic]
fn test_from_vec_with_len_equal_to_cap() {
    let mut vec: Vec<u8> = Vec::with_capacity(5);
    vec.extend_from_slice(&[1, 2, 3, 4, 5]); // len = 5, cap = 5
    let bytes = Bytes::from(vec);
}

#[test]
fn test_from_vec_with_aligned_pointer() {
    let vec: Vec<u8> = vec![1, 2, 3, 4]; // len = 4, cap = default capacity
    let alignment_check = {
        let bytes = Bytes::from(vec);
        let shared_ptr = bytes.data.load(Ordering::SeqCst);
        shared_ptr as usize & KIND_MASK == 0
    };
    assert!(alignment_check);
}

