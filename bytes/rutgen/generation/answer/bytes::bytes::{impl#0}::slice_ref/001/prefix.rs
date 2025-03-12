// Answer 0

#[test]
fn test_slice_ref_empty_subset() {
    let bytes = Bytes::from_static(b"hello world");
    let subset: &[u8] = &[];
    let subslice = bytes.slice_ref(subset);
}

#[test]
fn test_slice_ref_empty_subset_on_empty_bytes() {
    let bytes = Bytes::new();
    let subset: &[u8] = &[];
    let subslice = bytes.slice_ref(subset);
}

#[test]
fn test_slice_ref_empty_subset_on_non_empty_bytes() {
    let bytes = Bytes::copy_from_slice(b"example");
    let subset: &[u8] = &[];
    let subslice = bytes.slice_ref(subset);
}

