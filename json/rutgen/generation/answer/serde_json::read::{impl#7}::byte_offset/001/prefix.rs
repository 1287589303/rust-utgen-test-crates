// Answer 0

#[test]
fn test_byte_offset_empty_slice() {
    let slice: &[u8] = &[];
    let mut reader = SliceRead { slice, index: 0 };
    let _ = reader.byte_offset();
}

#[test]
fn test_byte_offset_single_element_slice() {
    let slice: &[u8] = &[1];
    let mut reader = SliceRead { slice, index: 0 };
    let _ = reader.byte_offset();
    
    reader.index = 1;
    let _ = reader.byte_offset();
}

#[test]
fn test_byte_offset_multiple_element_slice() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut reader = SliceRead { slice, index: 0 };
    let _ = reader.byte_offset();
    
    for i in 0..slice.len() {
        reader.index = i;
        let _ = reader.byte_offset();
    }
    
    reader.index = slice.len();
    let _ = reader.byte_offset();
}

#[test]
fn test_byte_offset_out_of_bounds() {
    let slice: &[u8] = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: 3 };
    let _ = reader.byte_offset();
}

