// Answer 0

#[test]
fn test_write_byte_index_zero() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.write_byte(0, 1);
}

#[test]
fn test_write_byte_index_one() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.write_byte(1, 2);
}

#[test]
fn test_write_byte_index_two() {
    let mut data = [0u8; 3];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 3) };
    slice.write_byte(2, 3);
}

#[test]
fn test_write_byte_boundary_high() {
    let mut data = [0u8; 5];
    let slice = unsafe { UninitSlice::from_raw_parts_mut(data.as_mut_ptr(), 5) };
    slice.write_byte(4, 255);
}

