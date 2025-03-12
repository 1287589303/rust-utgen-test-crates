// Answer 0

#[test]
fn test_try_read_u16_as_usize_empty_slice() {
    let slice: &[u8] = &[];
    let what: &'static str = "empty slice";
    let result = try_read_u16_as_usize(slice, what);
}

#[test]
fn test_try_read_u16_as_usize_one_byte_slice() {
    let slice: &[u8] = &[0x01];
    let what: &'static str = "one byte slice";
    let result = try_read_u16_as_usize(slice, what);
}

#[test]
fn test_try_read_u16_as_usize_two_byte_slice_min_value() {
    let slice: &[u8] = &[0x00, 0x00];
    let what: &'static str = "two byte slice min value";
    let result = try_read_u16_as_usize(slice, what);
}

#[test]
fn test_try_read_u16_as_usize_two_byte_slice_mid_value() {
    let slice: &[u8] = &[0x7F, 0xFF];
    let what: &'static str = "two byte slice mid value";
    let result = try_read_u16_as_usize(slice, what);
}

#[test]
fn test_try_read_u16_as_usize_two_byte_slice_max_value() {
    let slice: &[u8] = &[0xFF, 0xFF];
    let what: &'static str = "two byte slice max value";
    let result = try_read_u16_as_usize(slice, what);
}

