// Answer 0

#[test]
#[should_panic]
fn test_write_byte_index_equal_length() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    
    slice.write_byte(3, 255);
}

#[test]
#[should_panic]
fn test_write_byte_index_equal_to_zero_length() {
    let mut data: [MaybeUninit<u8>; 0] = [];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 0) };
    
    slice.write_byte(0, 255);
}

