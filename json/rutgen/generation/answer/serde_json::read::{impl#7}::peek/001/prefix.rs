// Answer 0

#[test]
fn test_peek_first_element() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut reader = SliceRead { slice, index: 0 };
    let result = reader.peek();
}

#[test]
fn test_peek_middle_element() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut reader = SliceRead { slice, index: 2 };
    let result = reader.peek();
}

#[test]
fn test_peek_last_element() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let mut reader = SliceRead { slice, index: 4 };
    let result = reader.peek();
}

