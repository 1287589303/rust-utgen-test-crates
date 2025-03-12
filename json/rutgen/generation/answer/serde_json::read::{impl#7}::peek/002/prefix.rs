// Answer 0

#[test]
fn test_peek_empty_slice() {
    let slice: &[u8] = &[];
    let mut reader = SliceRead { slice, index: 0 };
    let result = reader.peek();
}

#[test]
fn test_peek_single_element_slice() {
    let slice: &[u8] = &[1];
    let mut reader = SliceRead { slice, index: 1 };
    let result = reader.peek();
}

#[test]
fn test_peek_multiple_elements_slice() {
    let slice: &[u8] = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: 3 };
    let result = reader.peek();
}

#[test]
fn test_peek_with_valid_index() {
    let slice: &[u8] = &[9, 8, 7];
    let mut reader = SliceRead { slice, index: 2 };
    let result = reader.peek();
}

