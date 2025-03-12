// Answer 0

#[test]
fn test_slice_ref_with_non_empty_subset_out_of_bounds_start() {
    let bytes = Bytes::from_static(b"012345678");

    let subset: &[u8] = &b"01"[..];
    
    let _ = bytes.slice_ref(subset);
}

#[test]
#[should_panic]
fn test_slice_ref_with_non_empty_subset_out_of_bounds_end() {
    let bytes = Bytes::from_static(b"012345678");
    
    let subset: &[u8] = &b"123456789"[..];
    
    let _ = bytes.slice_ref(subset);
}

