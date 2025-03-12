// Answer 0

#[test]
fn test_try_read_state_id_buffer_too_small_empty_slice() {
    let slice: &[u8] = &[];
    let what: &'static str = "test_empty_slice";
    let result = try_read_state_id(slice, what);
}

#[test]
fn test_try_read_state_id_buffer_too_small_one_byte() {
    let slice: &[u8] = &[0u8];
    let what: &'static str = "test_one_byte_slice";
    let result = try_read_state_id(slice, what);
}

#[test]
fn test_try_read_state_id_buffer_too_small_two_bytes() {
    let slice: &[u8] = &[0u8, 1u8];
    let what: &'static str = "test_two_bytes_slice";
    let result = try_read_state_id(slice, what);
}

#[test]
fn test_try_read_state_id_buffer_too_small_max_before_size() {
    let slice: &[u8] = &[0u8; StateID::SIZE - 1];
    let what: &'static str = "test_max_before_size_slice";
    let result = try_read_state_id(slice, what);
}

