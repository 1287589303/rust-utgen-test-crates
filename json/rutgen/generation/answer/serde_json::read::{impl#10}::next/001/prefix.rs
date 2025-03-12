// Answer 0

#[test]
fn test_next_with_non_empty_slice() {
    let slice: &[u8] = &b"Hello, world!"[..];
    let mut slice_read = SliceRead { slice, index: 0 };
    let mut str_read = StrRead { delegate: slice_read, data: "Hello, world!" };
    let _ = str_read.next();
}

#[test]
fn test_next_empty_slice() {
    let slice: &[u8] = &[];
    let mut slice_read = SliceRead { slice, index: 0 };
    let mut str_read = StrRead { delegate: slice_read, data: "" };
    let _ = str_read.next();
}

#[test]
fn test_next_at_end_of_slice() {
    let slice: &[u8] = &b"Hello"[..];
    let mut slice_read = SliceRead { slice, index: slice.len() }; // Set index to the end
    let mut str_read = StrRead { delegate: slice_read, data: "Hello" };
    let _ = str_read.next(); // Should return None
}

