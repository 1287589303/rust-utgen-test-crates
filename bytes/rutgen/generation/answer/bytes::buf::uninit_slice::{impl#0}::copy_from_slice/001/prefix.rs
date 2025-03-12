// Answer 0

#[test]
fn test_copy_from_slice_empty() {
    let mut data: [u8; 0] = [];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 0) };
    slice.copy_from_slice(&[]);
}

#[test]
fn test_copy_from_slice_single_element() {
    let mut data = [0u8; 1];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 1) };
    slice.copy_from_slice(b"b");
}

#[test]
fn test_copy_from_slice_multiple_elements() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.copy_from_slice(b"foo");
}

#[test]
#[should_panic]
fn test_copy_from_slice_mismatched_length() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.copy_from_slice(b"barb");
}

