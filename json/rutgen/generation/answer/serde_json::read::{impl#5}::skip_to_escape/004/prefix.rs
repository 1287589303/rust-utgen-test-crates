// Answer 0

#[test]
fn test_skip_to_escape_non_empty_slice_less_than_length() {
    let slice = &[0x21, 0x22, 0x23, 0x24]; // non-empty slice
    let mut reader = SliceRead::new(slice);
    reader.index = 1; // index is less than slice.len()
    let forbid_control_characters = true;
    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_correct_length_multiple_of_word_size() {
    let slice = &[0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28]; // length is multiple of 8
    let mut reader = SliceRead::new(slice);
    reader.index = 1; // index is less than slice.len()
    let forbid_control_characters = true;
    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_case_no_control_characters() {
    let slice = &[0x21, 0x22, 0x23, 0x24]; // no control characters
    let mut reader = SliceRead::new(slice);
    reader.index = 1; // index is less than slice.len()
    let forbid_control_characters = true;
    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_case_multiple_remaining_bytes() {
    let slice = &[0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28]; // length multiple of 8
    let mut reader = SliceRead::new(slice);
    reader.index = 1; // index is less than slice.len()
    let forbid_control_characters = true;
    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_reaching_end_of_slice() {
    let slice = &[0x21, 0x22, 0x23, 0x24, 0x25, 0x26]; // non-empty slice
    let mut reader = SliceRead::new(slice);
    reader.index = 4; // index is less than slice.len()
    let forbid_control_characters = true;
    reader.skip_to_escape(forbid_control_characters);
}

