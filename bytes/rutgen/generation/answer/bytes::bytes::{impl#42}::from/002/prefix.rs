// Answer 0

#[test]
fn test_from_vec_with_non_aligned_pointer() {
    let vec = vec![1, 2, 3]; // len < cap
    let bytes = Bytes::from(vec);
}

#[test]
fn test_from_vec_with_aligned_pointer() {
    let vec = Vec::with_capacity(10); // empty Vec with capacity > 0
    let bytes = Bytes::from(vec);
}

