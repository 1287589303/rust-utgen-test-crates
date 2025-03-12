// Answer 0

#[test]
fn test_skip_to_escape_empty_slice() {
    let slice: &[u8] = &[];
    let mut reader = SliceRead::new(slice);
    reader.index = reader.slice.len();
    reader.skip_to_escape(true);
}

#[test]
fn test_skip_to_escape_empty_slice_no_control() {
    let slice: &[u8] = &[];
    let mut reader = SliceRead::new(slice);
    reader.index = reader.slice.len();
    reader.skip_to_escape(false);
}

