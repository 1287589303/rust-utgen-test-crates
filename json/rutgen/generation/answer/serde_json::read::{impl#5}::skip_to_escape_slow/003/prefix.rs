// Answer 0

#[test]
fn test_skip_to_escape_slow_when_index_equals_slice_length() {
    let data: &[u8] = &[b'a', b'b', b'c']; // slice with elements
    let mut slice_reader = SliceRead::new(data);
    slice_reader.index = data.len(); // index set to slice length
    slice_reader.skip_to_escape_slow(); // call the function
}

#[test]
fn test_skip_to_escape_slow_with_non_escape_character_at_last_index() {
    let data: &[u8] = &[b'a', b'b', b'c']; // slice with elements
    let mut slice_reader = SliceRead::new(data);
    slice_reader.index = data.len(); // index set to slice length
    slice_reader.skip_to_escape_slow(); // call the function
}

