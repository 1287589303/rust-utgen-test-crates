// Answer 0

#[test]
fn test_next_empty_slice() {
    let mut slice_reader = SliceRead {
        slice: &[],
        index: 0,
    };
    let result = slice_reader.next();
}

#[test]
fn test_next_non_empty_slice_at_bound() {
    let byte_slice: &[u8] = &[1, 2, 3];
    let mut slice_reader = SliceRead {
        slice: byte_slice,
        index: byte_slice.len(),
    };
    let result = slice_reader.next();
}

#[test]
fn test_next_single_element_slice_at_bound() {
    let byte_slice: &[u8] = &[42];
    let mut slice_reader = SliceRead {
        slice: byte_slice,
        index: byte_slice.len(),
    };
    let result = slice_reader.next();
}

#[test]
fn test_next_two_element_slice_at_bound() {
    let byte_slice: &[u8] = &[5, 10];
    let mut slice_reader = SliceRead {
        slice: byte_slice,
        index: byte_slice.len(),
    };
    let result = slice_reader.next();
}

