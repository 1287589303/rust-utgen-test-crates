// Answer 0

#[test]
fn test_copy_from_slice_different_length_zero_length_src() {
    let mut data: [u8; 3] = [0; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.copy_from_slice(&[]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_different_length_non_matching_length() {
    let mut data: [u8; 3] = [0; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.copy_from_slice(&[1, 2]);
}

#[test]
#[should_panic]
fn test_copy_from_slice_different_length_source_longer() {
    let mut data: [u8; 3] = [0; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.copy_from_slice(&[1, 2, 3, 4]);
}

#[test]
fn test_copy_from_slice_different_length_non_zero_length_src() {
    let mut data: [u8; 4] = [0; 4];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 4) };
    slice.copy_from_slice(&[1, 2, 3, 4]);
}

