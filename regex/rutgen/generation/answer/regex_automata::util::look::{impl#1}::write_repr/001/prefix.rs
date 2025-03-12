// Answer 0

#[test]
fn test_write_repr_valid_slice_length_4() {
    let look_set = LookSet { bits: 0xFFFFFFFF };
    let mut slice = [0u8; 4];
    look_set.write_repr(&mut slice);
}

#[test]
fn test_write_repr_valid_slice_length_5() {
    let look_set = LookSet { bits: 0x12345678 };
    let mut slice = [0u8; 5];
    look_set.write_repr(&mut slice);
}

#[test]
fn test_write_repr_valid_slice_length_6() {
    let look_set = LookSet { bits: 0x87654321 };
    let mut slice = [0u8; 6];
    look_set.write_repr(&mut slice);
}

#[should_panic]
fn test_write_repr_invalid_slice_length_3() {
    let look_set = LookSet { bits: 0x0 };
    let mut slice = [0u8; 3];
    look_set.write_repr(&mut slice);
}

#[should_panic]
fn test_write_repr_invalid_empty_slice() {
    let look_set = LookSet { bits: 0x0 };
    let slice: &mut [u8] = &mut [];
    look_set.write_repr(slice);
}

